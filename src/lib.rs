#[macro_export]
macro_rules! svg{
    ($($t:tt)*)=>{{
        let mut res = String::new();
        svg_w!{res , $($t)*};
        res
    }};
}

#[macro_export]
macro_rules! svg_branch {
    ($($t:tt)*)=>{{
        let mut res = String::new();
        svg_branch_w!{res , $($t)*};
        res
    }};
}

#[macro_export]
macro_rules! svg_w{
    ($wr:ident,$($p:tt=$v:tt),* =>$($e:tt)+)=>(svg_branch_w!($wr,{
        svg $($p=$v),*,
        xmlns="http://www.w3.org/2000/svg",
        "xmlns:xlink"= "http://www.w3.org/1999/xlink"
            => $($e)*
    }));
}

#[macro_export]
macro_rules! svg_list{
    ($wr:ident,$h:tt)=>(svg_branch_w!($wr,$h));
    ($wr:ident,$h:tt  $($t:tt)*)=>({
        svg_branch_w!($wr,$h);
        svg_list!($wr,$($t)*);
    });
    ($wr:ident,$l:literal)=>{write!($wr,"{}",$l);};
    ($wr:ident,($i:ident))=>(write!($wr,"{}",stringify!($i)));
    ($wr:ident,($s:expr))=>(write!($wr,"{}",stringify!($s)));

}

//return &'static str with property name
#[macro_export]
macro_rules! svg_prop_name {
    (w) => {
        "width"
    };
    (h) => {
        "height"
    };
    ($i:ident) => {
        stringify!($i)
    };
    ($l:literal) => {
        $l
    };
    ($e:expr) => {
        $e
    };
}

//returns &'static str with property value
#[macro_export]
macro_rules! svg_property {
    ({ }) => {
        ""
    };
    ($l:literal) => {
        $l
    };
    (($e:expr)) => {
        $e
    };
    ($e:expr) => {
        stringify!($e)
    };
}

//writes to the writer given as $wr
#[macro_export]
macro_rules! svg_branch_w {
    ($wr:ident,$l:literal)=>{write!($wr,"{}",$l)};
    ($wr:ident,(@if let $p:pat = $e:expr => $($t:tt)*))=>{
        if let $p = $e{
            svg_list!($wr,$($t)*);
        }
    };
    ($wr:ident,(@if $e:expr => $($t:tt)*))=> {
        if $e{
            svg_list!($wr,$($t)*);
        }
    };
    ($wr:ident,(@for $i:tt in $e:expr => $($t:tt)*)) => {{
        for $i in $e {
            svg_list!($wr,$($t)*);
        }
    }};

    ($wr:ident,$n:ident $($p:tt=$v:tt),* => $($e:tt)*) => {
        {
        let mut props = String::new();
        $(
        props.push_str(&format!("{}=\"{}\" ",svg_prop_name!($p),svg_property!($v)));
        )*
        write!($wr,"<{} {}>",stringify!($n),props);
        svg_list!($wr,$($e)*);
        write!($wr,"</{}>",stringify!($n));
        }
    };
    ($wr:ident,$n:ident $($p:ident=$v:expr),*)=>{
        {
        let mut props = String::new();
        $(
        props.push_str(&format!("{}=\"{}\" ",stringify!($p),$v));
        )*
        write!($wr,"<{} {}/>",stringify!($n),props)
        }
    };
    ($wr:ident,{$n:ident $($p:ident=$v:expr),*})=>{
        svg_branch_w!{$wr,$n $($p=$v),*}
    };
    ($wr:ident,{$n:ident $($p:tt=$v:tt),* => $e:tt}) => {
        svg_branch_w!{$wr, $n $($p=$v),* => $e};
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;
    #[test]
    fn top_level() {
        assert_eq!(
            &svg! { w=5,h=7=>
                    {big x=4,y=5}
            },
            "<svg width=\"5\" height=\"7\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" ><big x=\"4\" y=\"5\" /></svg>"
        )
    }
    #[test]
    fn it_works() {
        assert_eq!(&svg_branch!({big x=4,y=(4+5)}), "<big x=\"4\" y=\"9\" />");
    }
    #[test]
    fn test_text() {
        assert_eq!(
            &svg_branch!({text x=4 => "hello"}),
            "<text x=\"4\" >hello</text>"
        );
    }

    #[test]
    fn test_control() {
        let a = true;
        assert_eq!(
            &svg_branch!(g x=4,y=3 =>
             ( @if a =>
                {rect x=3,y=7}
             )
            ),
            r#"<g x="4" y="3" ><rect x="3" y="7" /></g>"#
        );
        let a = false;
        assert_eq!(
            &svg_branch!(g x=4,y=3 => ( @if a =>
                {rect x=3,y=7}
            )),
            r#"<g x="4" y="3" ></g>"#
        );
    }

    #[test]
    fn test_if_let() {
        let a = Some(5);
        assert_eq!(
            &svg_branch!(g x=4,y=3 => ( @if let Some(n) = a =>
                {rect x=3,y=(n+2)}
            )
            ),
            r#"<g x="4" y="3" ><rect x="3" y="7" /></g>"#
        );
    }

    #[test]
    fn test_loop() {
        assert_eq!(
            &svg_branch!(g x=3 =>  (@for x in 0..3 =>
                {rect x=(x*2)}
            )),
            r#"<g x="3" ><rect x="0" /><rect x="2" /><rect x="4" /></g>"#
        );
    }
}

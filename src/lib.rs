#[macro_export]
macro_rules! svg{
    ($w:expr,$h:expr=>$e:tt)=>(svg_branch!({
        svg width=$w, height=$h,
        xmlns="http://www.w3.org/2000/svg",
        "xmlns:xlink"= "http://www.w3.org/1999/xlink"
            => $e
    }));
}

#[macro_export]
macro_rules! svg_list{
    ({$($t:tt),*})=>({
        let mut s = String::new();
        $(
            s.push_str(&svg_branch!($t));
        )*
        s
    });
    ($l:literal)=>{$l};
    (($i:ident))=>(stringify!($i));
    (($s:expr))=>(stringify!($s));

}

#[macro_export]
macro_rules! svg_prop_name {
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

#[macro_export]
macro_rules! svg_branch {
    ($l:literal)=>{stringify!($l)};
    ((@if let $p:pat = $e:expr => $t:tt))=>{
        if let $p = $e{
            svg_list!($t)
        }else {String::new()}
    };
    ((@if $e:expr => $t:tt))=> {
        if $e{
            svg_list!($t)
        }else {String::new()}
    };
    ((@for $i:tt in $e:expr => $t:tt)) => {{
        let mut res = String::new();
        for $i in $e {
            res.push_str(&svg_list!($t));
        }
        res
    }};

    ($n:ident $($p:tt=$v:tt),* => $e:tt) => {
        {
        let mut props = String::new();
        $(
        props.push_str(&format!("{}=\"{}\" ",svg_prop_name!($p),svg_property!($v)));
        )*
        format!("<{0} {1}>{2}</{0}>",stringify!($n), props,svg_list!($e))
        }
    };
    ($n:ident $($p:ident=$v:expr),*)=>{
        {
        let mut props = String::new();
        $(
        props.push_str(&format!("{}=\"{}\" ",stringify!($p),$v));
        )*
        format!("<{} {}/>",stringify!($n),props)
        }
    };
    ({$n:ident $($p:ident=$v:expr),*})=>{
        svg_branch!{$n $($p=$v),*}
    };
    ({$n:ident $($p:tt=$v:tt),* => $e:tt}) => {
        svg_branch!{$n $($p=$v),* => $e};
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn top_level() {
        assert_eq!(
            &svg! {
                5,7=>{
                    {big x=4,y=5}
                }
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
            &svg_branch!(g x=4,y=3 => {( @if a => {
                {rect x=3,y=7}
            }
            )}),
            r#"<g x="4" y="3" ><rect x="3" y="7" /></g>"#
        );
        let a = false;
        assert_eq!(
            &svg_branch!(g x=4,y=3 => {( @if a => {
                {rect x=3,y=7}
            }
            )}),
            r#"<g x="4" y="3" ></g>"#
        );
    }

    #[test]
    fn test_if_let() {
        let a = Some(5);
        assert_eq!(
            &svg_branch!(g x=4,y=3 => {( @if let Some(n) = a => {
                {rect x=3,y=(n+2)}
            }
            )}),
            r#"<g x="4" y="3" ><rect x="3" y="7" /></g>"#
        );
    }

    #[test]
    fn test_loop() {
        assert_eq!(
            &svg_branch!(g x=3 => { (@for x in 0..3 => {
                {rect x=(x*2)}
            }
            )}),
            r#"<g x="3" ><rect x="0" /><rect x="2" /><rect x="4" /></g>"#
        );
    }
}

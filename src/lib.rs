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
    ({$n:ident $($p:tt=$v:tt),* => $e:tt}) => {
        {
        let mut props = String::new();
        $(
        props.push_str(&format!("{}=\"{}\" ",svg_prop_name!($p),svg_property!($v)));
        )*
        format!("<{0} {1}>{2}</{0}>",stringify!($n), props,svg_list!($e))
        }
    };
    ({$n:ident $($p:ident=$v:expr),*;})=>{
        {
        let mut props = String::new();
        $(
        props.push_str(&format!("{}=\"{}\" ",stringify!($p),$v));
        )*
        format!("<{} {}/>",stringify!($n),props)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn top_level() {
        assert_eq!(
            &svg! {
                5,7=>{
                    {big x=4,y=5 ;}
                }
            },
            "<svg width=\"5\" height=\"7\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" ><big x=\"4\" y=\"5\" /></svg>"
        )
    }
    #[test]
    fn it_works() {
        assert_eq!(&svg_branch!({big x=4,y=(4+5) ;}), "<big x=\"4\" y=\"9\" />");
    }
    #[test]
    fn test_text() {
        assert_eq!(
            &svg_branch!({text x=4 => "hello"}),
            "<text x=\"4\" >hello</text>"
        );
    }
}

use std::collections::HashMap;

use crate::html::{Builder, Stringify};
use paste::paste;

pub trait NodeLike {
    fn parent(&self) -> &HtmlNode;
    fn children(&self) -> &Vec<HtmlNode>;
}

#[derive(Debug, Clone, Default)]
pub struct HtmlNode {
    parent: Box<HtmlNode>,
    children: Box<Vec<HtmlNode>>,
}
impl NodeLike for HtmlNode {
    fn parent(&self) -> &HtmlNode { self.parent.as_ref() }
    fn children(&self) -> &Vec<HtmlNode> { self.children.as_ref() }
}
impl HtmlNode {
    fn new() -> Self { Default::default() }
}

macro_rules! tag_builder {
    ( $impl_name:ident [ $( $fn_name:ident ),*  ] ) => {
        paste! {
            pub struct $impl_name { attrs: HashMap<String, String> }
            pub struct [<$impl_name Builder>] { attrs: HashMap<String, String>  }

            impl $impl_name {
                // This implementation will change to improve performance
                $( pub fn $fn_name(&self) -> &str { self.attrs.get(stringify!($fn_name)).unwrap()} )*
            }
            
            impl [<$impl_name Builder>] {
                $( pub fn $fn_name(mut self, value: &str) -> Self {
                    eprintln!("Created: {}", stringify!($fn_name));
                    self.attrs.insert(stringify!($fn_name).into(), value.to_string());
                    self
                } )*
            }

            impl $impl_name {
                pub fn new() -> [<$impl_name Builder>] { Default::default() }
            }

            impl Default for [<$impl_name Builder>] {
                fn default() -> Self {
                    let mut attrs = HashMap::new();
                    $( attrs.insert(stringify!($fn_name).into(), "".into()); )*
                    Self { attrs }
                }
            }

            impl Builder<$impl_name> for [<$impl_name Builder>] {
                fn build(self) -> $impl_name { $impl_name { attrs: self.attrs } }
            }

            impl Stringify for $impl_name {
                fn stringify(&self) -> String {
                    let mut s = String::new();
                    self.attrs.iter().for_each(|(key, val)| {
                        if !val.is_empty() {
                            s.push_str(format!("{}=\"{}\" ", key, val).as_str());
                        }
                    });
            
                    if s.is_empty() { "".to_string() } else {
                        format!("\n\t<s {}>", s)
                    }
                }
            }
        }
    };
}
tag_builder!(LinkTag[rel, href, target, media, crossorigin, as_, sizes, referrerpolicy, coords, shape, ping]);



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn work() {
        let rel = "attachment";
        let href = "https://best-drop.com";
        let crossorigin = "cross-origin";

        let link = LinkTag::new()
            .rel(rel)
            .href(href)
            .crossorigin(crossorigin)
            .build();

        assert_eq!(link.rel(), rel);
        assert_eq!(link.href(), href);
        assert_eq!(link.crossorigin(), crossorigin);
    }
}

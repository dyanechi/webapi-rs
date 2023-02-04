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

            // This implementation will change to improve performance
            impl $impl_name {
                $( pub fn $fn_name(&self) -> &str { self.attrs.get(stringify!($fn_name)).unwrap()} )*
            }

            impl $impl_name { pub fn new() -> [<$impl_name Builder>] { Default::default() } }
            
            impl [<$impl_name Builder>] {
                $( pub fn $fn_name(mut self, value: &str) -> Self {
                    eprintln!("Created: {}", stringify!($fn_name));
                    self.attrs.insert(stringify!($fn_name).into(), value.to_string());
                    self
                } )*
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


// pub struct LinkTagBuilder {
//     // rel: String,
//     // href: String,
//     // target: String,
//     // media: String,
//        shape: String,
//     attrs: HashMap<String, String>
// }
// impl Default for LinkTagBuilder {
//     fn default() -> Self {
//         let mut attrs = HashMap::new();
//         attrs.insert("rel".into(), "".into());
//         attrs.insert("href".into(), "".into());
//         attrs.insert("target".into(), "".into());
//         attrs.insert("media".into(), "".into());
//         Self { attrs }
//     }
// }
// impl Builder<LinkTag> for LinkTagBuilder {
//     fn build(self) -> LinkTag { LinkTag { attrs: self.attrs } }
// }

// impl LinkTagBuilder {
//     pub fn rel(mut self, value: &str) -> Self { self.attrs.insert("rel".into(), value.into()); self }
//     pub fn href(mut self, value: &str) -> Self { self.attrs.insert("href".into(), value.into()); self }
//     pub fn target(mut self, value: &str) -> Self { self.attrs.insert("target".into(), value.into()); self }
//     pub fn media(mut self, value: &str) -> Self { self.attrs.insert("media".into(), value.into()); self }
//     tag_setter!(crossorigin(value: &str));
//     tag_setter_fn!(as_);
// }


// pub struct LinkTag {
//     attrs: HashMap<String, String>
// }
// impl LinkTag {
//     pub fn new() -> LinkTagBuilder { Default::default() }
// }
// impl Stringify for LinkTag {
//     fn stringify(&self) -> String {
//         let mut link = String::new();
//         self.attrs.iter().for_each(|(key, val)| {
//             if !val.is_empty() {
//                 link.push_str(format!("{}=\"{}\" ", key, val).as_str());
//             }
//         });

//         if link.is_empty() { "".to_string() } else {
//             format!("\n\t<link {}>", link)
//         }
//     }
// }
// impl LinkTag {
//     pub fn rel(&self) -> &str { self.attrs.get("rel").unwrap() }
//     pub fn href(&self) -> &str { self.attrs.get("href").unwrap() }
//     pub fn target(&self) -> &str { self.attrs.get("target").unwrap() }
//     pub fn media(&self) -> &str { self.attrs.get("media").unwrap() }
// }


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

pub use super::{doc::*, head::*, body::*, link::*};
use paste::paste;

macro_rules! tag_builder_bc {
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

macro_rules! build_tag {
    ( $( $tag_name:ident),* $(,)* ) => {
        paste! {
            $( pub struct [<$tag_name Tag>] {} )*
        }
    }
}

macro_rules! rsx {
    () => {};
    (
        $($tag:ident { })*
    ) => {
        
    };

    ( $($tag:ident { $($child_tag:ident { $($tail:tt)* })* })* ) => {
        $( rsx!($($tail)*) )*;
    };
}

build_tag!(
    A,
    Area,
    Audio,
    Bdi,
    BlockQuote,
    Body,
    Br,
    Button,
    Canvas,
    Caption,
    Cite,
    Code,
    Col,
    ColGroup,
    Command,
    DataList,
    Dd,
    Del,
    Details,
    Dfn,
    Dialog,
    Div,
    Dl,
    Dt,
    Em,
    Embed,
    FieldSet,
    FigCaption,
    Figure,
    Footer,
    Form,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Head,
    Header,
    Hr,
    Html,
    I,
    IFrame,
    Img,
    Input,
    Ins,
    Kbd,
    Label,
    Legend,
    Li,
    Link,
    Main,
    Map,
    Mark,
    Menu,
    Meta,
    Meter,
    Nav,
    NoScript,
    Object,
    Ol,
    OptGroup,
    Option,
    Output,
    P,
    Param,
    Picture,
    Pre,
    Progress,
    Q,
    Rb,
    Rp,
    Rt,
    Rtc,
    Ruby,
    S,
    SAmp,
    Script,
    Section,
    Select,
    Small,
    Source,
    Span,
    Strong,
    Style,
    Sub,
    Summary,
    Sup,
    Table,
    TBody,
    Td,
    Template,
    Textarea,
    TFoot,
    Th,
    THead,
    Time,
    Title,
    Tr,
    Track,
    U,
    Ul,
    Var,
    Video,
    Wbr,
);


rsx!(
    Div {
        Div {
            Div {

            }
        }
    }
);



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tags_build() {
        let option = OptionTag {};
    }
}
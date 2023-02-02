use std::ops::Deref;

// trait HtmlElement { }

#[derive(Default)]
struct CssStyle {}

#[derive(Default, Clone)]
struct HtmlInlineAttr {
    id: String,
    style: &'static[CssStyle],
    class: &'static[&'static str],
    width: u32,
    height: u32,
}
impl HtmlInlineAttr {
    
}

#[derive(Default)]
struct Element {
    // child: C,
    pub id: String,
    pub inner_html: String,
    pub class_name: String,
    pub client_width: u32,
    pub client_height: u32,
    pub client_top: u32,
    pub client_left: u32,
    pub client_right: u32,
    pub client_bottom: u32,
}
// impl Deref for Element {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         &self.child
//     }
// }
// impl<C> Default for Element<C> {
//     fn default() -> Element<C> {
//         Element::<C>::default()
//     }
// }


#[derive(Default)]
struct HtmlElement {
    // child: C,
    pub inner_text: String,
    pub hidden: bool,
    pub title: String,
    pub nonce: u8,
    pub offset_width: u32,
    pub offset_height: u32,
    pub offset_top: u32,
    pub offset_left: u32,
    pub offset_parent: Box<HtmlElement>,
}
// impl<T> Deref for HtmlElement<T> {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         &self.child
//     }
// }

macro_rules! create_tag {
    ($name:ident { $( $field:ident : $type:ty ),* $(,)* }) => {
        struct $name {
            el_attr: Element,
            html_attr: HtmlElement,
            $( $field: $type, )*
        }
        impl Default for $name {
            fn default() -> Self {
                Self {
                    el_attr: Default::default(),
                    html_attr: Default::default(),
                    $( $field: Default::default(), )*
                }
            }
        }
    };
}

create_tag!(Div {
    content: String,
});

#[derive(Default)]
pub struct DivElement {
    width: u32,
    height: u32,
}

// type HtmlElem<T> = Element<HtmlElement<T>>;

// impl DivElement {
//     fn new() -> HtmlElem<Self> {
//         HtmlElem::<DivElement>::default()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn create_div() {
        // let el = DivElement::new();
        let div = Div::default();
        
        assert_eq!(div.content, "")
    }
}

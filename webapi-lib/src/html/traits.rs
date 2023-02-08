pub trait Buildable {
    fn new() -> dyn Builder<Self>;
}
pub trait Builder<T> {
    fn build(self) -> T;
}

pub trait Stringify {
    fn stringify(&self) -> String;
}

pub trait Render {
    fn render(&self);
}

pub trait Component: Render + Stringify {
    fn append(self, other: impl Component);
}

pub struct Comp {}
impl Render for Comp {
    fn render(&self) { }
}
impl Stringify for Comp {
    fn stringify(&self) -> String {
        "".into()
    }
}
impl Component for Comp {
    fn append(self, other: impl Component) {}
}

pub trait HtmlElement {}
pub trait HtmlEvent {}
pub trait HtmlNode {}


pub trait CssStyle {}
pub trait WebScript {}
pub trait Deploy {
    fn deploy(self) -> Self; 
}
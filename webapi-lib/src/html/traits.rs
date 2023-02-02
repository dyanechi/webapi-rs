pub trait Buildable {
    fn new() -> dyn Builder<Self>;
}
pub trait Builder<T> {
    fn build(self) -> T;
}

pub trait Stringify {
    fn stringify(&self) -> String;
}

pub trait HtmlElement {}
pub trait HtmlEvent {}
pub trait HtmlNode {}


pub trait CssStyle {}
pub trait WebScript {}
pub trait Deploy {
    fn deploy(self) -> Self; 
}
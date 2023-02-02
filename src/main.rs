use webapi_rs::html::{self, Deploy, tags::MetaKind};

fn main() {
    println!("Hello, world!");

    let head = html::HtmlHead::default()
        .add_meta(MetaKind::Author("dyanechi"))
        .add_meta(MetaKind::Description("Website developed with pure Rust"))
        .add_meta(MetaKind::Keywords("rust, website, pure"))
        .add_meta(MetaKind::Custom("ad-tracker", "BN39B35490-45-0"));
        
    let body = html::HtmlBody::default();

    html::HtmlDoc::builder()
        .head(head)
        .body(body)
        .styles(&[])
        .scripts(&[])
        .build()
        .deploy();

        
    // println!("{}", document.stringify());
}

use webapi_rs::html::{self, Deploy, tags::MetaKind};

fn main() {
    let head = html::HtmlHead::default()
        .author("dyanechi")
        .description("Website developed with pure Rust")
        .keywords("rust, web, api")
        .meta(MetaKind::OgEmail("support@email.com"))
        .meta(MetaKind::Custom("ad-tracker", "BN39B35490-45-0"));
        
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

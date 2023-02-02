use webapi_lib::html::{self, Deploy, tags::MetaKind, Builder};

fn main() {
    let head = html::HtmlHead::new()
        .title("Rust Framework Test")
        .author("dyanechi")
        .description("Website developed with pure Rust")
        .keywords("rust, web, api")
        .meta(MetaKind::OgEmail("support@email.com"))
        .meta(MetaKind::Custom("ad-tracker", "BN39B35490-45-0"))
        .build();
        
    let body = html::HtmlBody::default();

    let document = html::HtmlDoc::builder()
        .head(head)
        .body(body)
        .styles(&[])
        .scripts(&[])
        .build()
        .deploy();

    println!("Deployed!");
}

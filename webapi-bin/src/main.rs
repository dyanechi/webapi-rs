use webapi_lib::html::{self, Deploy, tags::MetaKind, Builder, LinkTag};

fn main() {
    let head = html::HtmlHead::builder()
        .title("Rust Framework Test")
        .author("dyanechi")
        .description("Website developed with pure Rust")
        .keywords("rust, web, api")
        .meta(MetaKind::OgEmail("support@email.com"))
        .meta(MetaKind::Custom("ad-tracker", "BN39B35490-45-0"))
        .link(
            LinkTag::new().rel("pizza").href("https://yomamma.com").build()
        )
        .link(
            LinkTag::new().rel("fries").href("https://yomamma.com").build()
        )
        // .link()
        //     .rel("Hamburger")
        //     .href("https://yomamma.com")
        //     .append()
        // .link(|o| o
        //     .rel("Hamburger")
        //     .href("https://yomamma.com")
        //     .append()
        // )
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

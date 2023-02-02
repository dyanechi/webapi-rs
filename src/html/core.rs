use std::{fmt::Display, fs, cell::RefCell};

use super::tags::*;

#[derive(Debug)]
pub enum WebApiErr {
    DeployErr(&'static str),
}

trait Stringify {
    fn stringify(&self) -> String;
}

trait HtmlElement {}
trait HtmlEvent {}
trait HtmlNode {}


pub trait CssStyle {}
pub trait WebScript {}
pub trait Deploy {
    fn deploy(self) -> Self; 
}

type CssStyles = &'static[Box<dyn CssStyle>];
type WebScripts = &'static[Box<dyn WebScript>];


/// Html Document Element
#[derive(Default)]
pub struct HtmlDoc {
    metadata: HtmlMetadata,
    head: HtmlHead,
    body: HtmlBody,
    styles: &'static[Box<dyn CssStyle>],
    scripts: &'static[Box<dyn WebScript>],
    str: Option<String>,
}
impl HtmlDoc {
    pub fn builder() -> HtmlDocBuilder {
        HtmlDocBuilder { html: HtmlDoc::default() }
    }
}
impl Stringify for HtmlDoc {
    fn stringify(&self) -> String {
        format!("{}", self.str.as_ref().expect("should be deployed"))
    }
}
impl Deploy for HtmlDoc {
    fn deploy(mut self) -> Self {
        use std::io::prelude::*;
        let root: String = format!(
            "<!DOCTYPE html>\n<html>{}{}\n</html>",
            self.head.stringify(),
            self.body.stringify(),
        );
        let path = "./public/index.html"; 
        match fs::remove_file(path) {
            _ => match fs::File::create(path) {
                Ok(mut f) => {
                    f.write_all(root.as_bytes()).unwrap();
                },
                Err(_) => { eprintln!("Failed to deploy Html"); }
            }
        };
        self.str = Some(root);
        self
    }
}
impl Display for HtmlDoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stringify())
        // Ok(())
    }
}


pub struct HtmlDocBuilder { html: HtmlDoc }
impl HtmlDocBuilder {
    pub fn build(self) -> HtmlDoc { self.html }

    pub fn metadata(mut self, metadata: HtmlMetadata) -> HtmlDocBuilder { self.html.metadata = metadata; self }
    pub fn head(mut self, head: HtmlHead) -> HtmlDocBuilder { self.html.head = head; self }
    pub fn body(mut self, body: HtmlBody) -> HtmlDocBuilder { self.html.body = body; self }
    pub fn styles(mut self, styles: CssStyles) -> HtmlDocBuilder { self.html.styles = styles; self }
    pub fn scripts(mut self, scripts: WebScripts) -> HtmlDocBuilder { self.html.scripts = scripts; self }
}

#[derive(Default, Clone)]
pub struct HtmlMetadata { }


pub struct HtmlHead {
    title: String,
    charset: String,
    meta: RefCell<Vec<MetaTag>>,
}
impl Default for HtmlHead {
    fn default() -> Self {
        Self {
            title: "WebApi Test Website".into(),
            charset: "utf-8".into(),
            meta: RefCell::new(vec![]),
        }
    }
}
impl Stringify for HtmlHead {
    fn stringify(&self) -> String {
        // let title = match self.title.as_ref() {
        //     Some(x) => x,
        //     None => ""
        // };
        // let charset = match self.charset.as_ref() {
        //     Some(x) => x,
        //     None => "utf-8"
        // };
        // let meta_tags: String = match self.meta.as_ref() {
        //     Some(x) => x.borrow().iter().map(|m| format!(
        //         "\n\t<meta {}=\"{}\" content=\"{}\" />", m.key(), m.value(), m.content()
        //     )).collect(),
        //     None => "".into(),
        // };
        let title: String = format!("\n\t<title>{}</title>", self.title);
        let charset: String = format!("\n\t<meta charset=\"{}\" />", self.charset);
        let meta_tags: String = self.meta.borrow().iter().map(|m| format!(
            "\n\t<meta {}=\"{}\" content=\"{}\" />", m.key(), m.value(), m.content()
        )).collect();

        format!(
            "<head>{}{}{}\n</head>",
            title,
            charset,
            meta_tags,
        ).lines().map(|line| format!("\n\t{line}")).collect()
    }
}
impl HtmlHead {
    pub fn title(mut self, title: &str) -> Self { self.title = title.into(); self }
    pub fn charset(mut self, charset: &str) -> Self { self.charset = charset.into(); self }
    // pub fn with_meta(mut self, meta: Vec<MetaTag>) -> Self { self.meta = Some(RefCell::new(meta)); self }

    pub fn meta(mut self, kind: MetaKind) -> Self {
        // if let None = self.meta { self.meta = Some(RefCell::new(vec![])) }
        self.meta
            .get_mut()
            .push(MetaTag::from_kind(kind));
        self
    }
}
impl HtmlHead {
    pub fn custom_meta(mut self, key: &str, value: &str, content: &str) -> Self {
        self.meta
            .get_mut()
            .push(MetaTag::new(key, value, content));
        self
    }
}
impl HtmlHead {
    pub fn abstract_(self, value: &'static str) -> Self { self.meta(MetaKind::Abstract(value)) }
    pub fn author(self, value: &'static str) -> Self { self.meta(MetaKind::Author(value)) }
    pub fn category(self, value: &'static str) -> Self { self.meta(MetaKind::Category(value)) }
    pub fn classification(self, value: &'static str) -> Self { self.meta(MetaKind::Classification(value)) }
    pub fn copyright(self, value: &'static str) -> Self { self.meta(MetaKind::Copyright(value)) }
    pub fn coverage(self, value: &'static str) -> Self { self.meta(MetaKind::Coverage(value)) }
    pub fn description(self, value: &'static str) -> Self { self.meta(MetaKind::Description(value)) }
    pub fn designer(self, value: &'static str) -> Self { self.meta(MetaKind::Designer(value)) }
    pub fn directory(self, value: &'static str) -> Self { self.meta(MetaKind::Directory(value)) }
    pub fn distribution(self, value: &'static str) -> Self { self.meta(MetaKind::Distribution(value)) }
    pub fn identifier_url(self, value: &'static str) -> Self { self.meta(MetaKind::IdentifierUrl(value)) }
    pub fn keywords(self, value: &'static str) -> Self { self.meta(MetaKind::Keywords(value)) }
    pub fn language(self, value: &'static str) -> Self { self.meta(MetaKind::Language(value)) }
    pub fn owner(self, value: &'static str) -> Self { self.meta(MetaKind::Owner(value)) }
    pub fn rating(self, value: &'static str) -> Self { self.meta(MetaKind::Rating(value)) }
    pub fn reply_to(self, value: &'static str) -> Self { self.meta(MetaKind::ReplyTo(value)) }
    pub fn revised(self, value: &'static str) -> Self { self.meta(MetaKind::Revised(value)) }
    pub fn revisit_after(self, value: &'static str) -> Self { self.meta(MetaKind::RevisitAfter(value)) }
    pub fn robots(self, value: &'static str) -> Self { self.meta(MetaKind::Robots(value)) }
    pub fn subject(self, value: &'static str) -> Self { self.meta(MetaKind::Subject(value)) }
    pub fn summary(self, value: &'static str) -> Self { self.meta(MetaKind::Summary(value)) }
    pub fn topic(self, value: &'static str) -> Self { self.meta(MetaKind::Topic(value)) }
    pub fn url(self, value: &'static str) -> Self { self.meta(MetaKind::Url(value)) }
    
}

#[derive(Default)]
pub struct HtmlBody {
    elements: Option<Vec<Box<dyn HtmlElement>>>,
}
impl Stringify for HtmlBody {
    fn stringify(&self) -> String {
        let elements: String = match self.elements.as_ref() {
            Some(x) => "".into(),
            None => "".into(),
        }; 
        format!(
            "<body>\n{}\n</body>",
            "Hello World!",
        ).lines().map(|line| format!("\n\t{line}")).collect()
    }
}
impl HtmlBody {
    pub fn build(self) -> String {
        let elements = self.elements.unwrap_or_default();
        format!(
            "<body>\n\t{}\n</body>",
            "Hello World!",
        )
    }
}

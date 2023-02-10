use std::{cell::RefCell, collections::HashMap, vec};

use super::*;

pub struct HtmlHeadBuilder {
    title: String,
    charset: String,
    meta: RefCell<Vec<MetaTag>>,
    link: RefCell<Vec<LinkTag>>,
}
impl Builder<HtmlHead> for HtmlHeadBuilder {
    fn build(self) -> HtmlHead {
        let mut meta: HashMap<String, MetaTag> = HashMap::new();
        // let link: RefCell<Vec<LinkTag>> = RefCell::new(vec![]);

        for tag in self.meta.take() {
            meta.insert(tag.value().to_string(), tag);
        }

        HtmlHead { title: self.title, charset: self.charset, meta, link: self.link }
    }
}
impl Default for HtmlHeadBuilder {
    fn default() -> Self {
        Self {
            title: "WebApi Test Website".into(),
            charset: "utf-8".into(),
            meta: RefCell::new(vec![]),
            link: RefCell::new(vec![]),
        }
    }
}
impl HtmlHeadBuilder {
    pub fn title(mut self, title: &str) -> Self { self.title = title.into(); self }
    pub fn charset(mut self, charset: &str) -> Self { self.charset = charset.into(); self }
    pub fn meta(mut self, kind: MetaKind) -> Self { self.meta.get_mut().push(MetaTag::from_kind(kind)); self }
    pub fn link(mut self, link: LinkTag) -> Self { self.link.get_mut().push(link); self }
}
impl HtmlHeadBuilder {
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
    
    pub fn http_equiv(self, key: &'static str, value: &'static str) -> Self { self.meta(MetaKind::HttpEquiv(key, value)) }
    pub fn property(self, key: &'static str, value: &'static str) -> Self { self.meta(MetaKind::Property(key, value)) }
    pub fn custom_meta(self, key: &'static str, value: &'static str) -> Self { self.meta(MetaKind::Custom(key, value)) }
}



pub struct HtmlHead {
    title: String,
    charset: String,
    meta: HashMap<String, MetaTag>,
    link: RefCell<Vec<LinkTag>>,
}
impl HtmlHead {
    pub fn builder() -> HtmlHeadBuilder {
        HtmlHeadBuilder::default()
    }
}
impl Default for HtmlHead {
    fn default() -> Self { HtmlHead::builder().build() }
}
impl Stringify for HtmlHead {
    fn stringify(&self) -> String {
        let title: String = format!("\n\t<title>{}</title>", self.title);
        let charset: String = format!("\n\t<meta charset=\"{}\" />", self.charset);
        let meta_tags: String = self.meta.values().map(|m| format!(
            "\n\t<meta {}=\"{}\" content=\"{}\" />",
            m.key(), m.value(), m.content()
        )).collect();

        let link_tags: String = self.link
            .borrow()
            .iter()
            .map(|l| l.stringify())
            .collect();

        format!(
            "<head>{}{}{}{}\n</head>",
            title, charset, meta_tags, link_tags,
        ).lines().map(|line| format!("\n\t{line}")).collect()
    }
}
impl HtmlHead {
    pub fn title(&self) -> &str { self.title.as_str() }
    pub fn charset(&self) -> &str { self.charset.as_str() }

    pub fn meta(&self, key: &str) -> &str {
        match self.meta.get(key) {
            Some(meta) => meta.content(),
            None => ""
        }
    }
    pub fn meta_mut(&mut self, key: &str) -> Option<&mut MetaTag> {
        self.meta.get_mut(key)
    }
}
impl HtmlHead {
    pub fn abstract_(&self) -> &str { self.meta("abstract") }
    pub fn author(&self) -> &str { self.meta("author") }
    pub fn category(&self) -> &str { self.meta("category") }
    pub fn classification(&self) -> &str { self.meta("classification") }
    pub fn copyright(&self) -> &str { self.meta("copyright") }
    pub fn coverage(&self) -> &str { self.meta("coverage") }
    pub fn description(&self) -> &str { self.meta("description") }
    pub fn designer(&self) -> &str { self.meta("designer") }
    pub fn directory(&self) -> &str { self.meta("directory") }
    pub fn distribution(&self) -> &str { self.meta("distribution") }
    pub fn identifier_url(&self) -> &str { self.meta("identifier-url") }
    pub fn keywords(&self) -> &str { self.meta("keywords") }
    pub fn language(&self) -> &str { self.meta("language") }
    pub fn owner(&self) -> &str { self.meta("owner") }
    pub fn rating(&self) -> &str { self.meta("rating") }
    pub fn reply_to(&self) -> &str { self.meta("reply-to") }
    pub fn revised(&self) -> &str { self.meta("revised") }
    pub fn revisit_after(&self) -> &str { self.meta("revisit-after") }
    pub fn robots(&self) -> &str { self.meta("robots") }
    pub fn subject(&self) -> &str { self.meta("subject") }
    pub fn summary(&self) -> &str { self.meta("summary") }
    pub fn topic(&self) -> &str { self.meta("topic") }
    pub fn url(&self) -> &str { self.meta("url") }

    pub fn http_equiv(&self, key: &'static str) -> &str { self.meta(key) }
    pub fn property(&self, key: &'static str) -> &str { self.meta(key) }
    pub fn custom_meta(&self, key: &'static str) -> &str { self.meta(key) }
}

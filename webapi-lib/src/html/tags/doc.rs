use std::{fs, fmt::Display};

use super::*;

#[derive(Debug)]
pub enum WebApiErr {
    DeployErr(&'static str),
}

type CssStyles = &'static[Box<dyn CssStyle>];
type WebScripts = &'static[Box<dyn WebScript>];



/// Html Document Element
#[derive(Default)]
pub struct HtmlDoc {
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
        format!("{}", self.str.as_ref().expect("should be deployed").to_string())
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

    pub fn head(mut self, head: HtmlHead) -> HtmlDocBuilder { self.html.head = head; self }
    pub fn body(mut self, body: HtmlBody) -> HtmlDocBuilder { self.html.body = body; self }
    pub fn styles(mut self, styles: CssStyles) -> HtmlDocBuilder { self.html.styles = styles; self }
    pub fn scripts(mut self, scripts: WebScripts) -> HtmlDocBuilder { self.html.scripts = scripts; self }
}
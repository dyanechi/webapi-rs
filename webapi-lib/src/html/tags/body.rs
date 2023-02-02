use super::*;

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
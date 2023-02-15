use super::{HtmlBody, HtmlDoc};



pub fn create_root(id: &'static str) -> HtmlDoc {
    HtmlDoc::builder()
    .build()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn build_head() {
//         let author = "test-author";
//         let head = HtmlHead::new()
//             .author(author)
//             .build();

//         assert_eq!(head.author(), author);
//         assert_eq!(head.charset(), "utf-8", "should have default charset");
//     }
// }
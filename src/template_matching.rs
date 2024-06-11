use pulldown_cmark::{Tag, TagEnd};

pub fn match_tag(tag: &Tag) -> Option<String> {
    return match tag {
        Tag::Paragraph => Some(String::from("p")),
        Tag::Heading { level, .. } => Some(format!("h{}", *level as u8)),
        Tag::BlockQuote(_) => Some(String::from("blockquote")),
        Tag::CodeBlock(_) => Some(String::from("code")),
        Tag::HtmlBlock => Some(String::from("html")),
        Tag::List(number) => {
            if number.is_some() {
                Some(String::from("ol"))
            } else {
                Some(String::from("ul"))
            }
        }
        Tag::Item => Some(String::from("li")),
        Tag::Table(_) => Some(String::from("table")),
        Tag::TableHead => Some(String::from("thead")),
        Tag::TableRow => Some(String::from("tr")),
        Tag::TableCell => Some(String::from("td")),
        Tag::Emphasis => Some(String::from("em")),
        Tag::Strong => Some(String::from("strong")),
        Tag::Strikethrough => Some(String::from("s")),
        Tag::Link { .. } => Some(String::from("a")),
        Tag::Image { .. } => Some(String::from("img")),
        Tag::FootnoteDefinition(_) => Some(String::from("footnote")),
        _ => None,
    };
}

pub fn match_tag_end(tag_end: &TagEnd) -> Option<String> {
    match tag_end {
        TagEnd::Paragraph => Some(String::from("p")),
        TagEnd::Heading(level) => Some(format!("h{}", *level as u8)),

        TagEnd::BlockQuote => Some(String::from("blockquote")),
        TagEnd::CodeBlock => Some(String::from("code")),

        TagEnd::HtmlBlock => Some(String::from("html")),

        TagEnd::List(is_ordered) => Some(String::from(if *is_ordered { "ol" } else { "ul" })),
        TagEnd::Item => Some(String::from("li")),
        TagEnd::FootnoteDefinition => Some(String::from("div")),

        TagEnd::Table => Some(String::from("table")),
        TagEnd::TableHead => Some(String::from("thead")),
        TagEnd::TableRow => Some(String::from("tr")),
        TagEnd::TableCell => Some(String::from("td")),

        TagEnd::Emphasis => Some(String::from("em")),
        TagEnd::Strong => Some(String::from("strong")),
        TagEnd::Strikethrough => Some(String::from("del")),

        TagEnd::Link => Some(String::from("a")),
        TagEnd::Image => Some(String::from("img")),
        _ => None,
    }
}

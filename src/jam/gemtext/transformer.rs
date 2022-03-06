use crate::jam::gemtext::scanner::*;

pub struct GemTransformer {
    pub item_list: Vec<GemTextToken>,
    pub html_list: Vec<String>,
}

// Gemtext -> HTML converter
// TODO: Add option to build html version gemlog
impl GemTransformer {
    pub fn new(item_list: Vec<GemTextToken>) -> GemTransformer {
        Self {
            item_list,
            html_list: Vec::new(),
        }
    }

    pub fn print_items(&self) {
        for item in &self.html_list {
            println!("{:?}", item);
        }
    }

    pub fn transform(&mut self) {
        for item in self.item_list.iter() {
            match &item._type {
                GemTextTokenType::Heading { level, text } => {
                    println!("<h{0}>{1}<h{0}>", level, text);
                }

                GemTextTokenType::List(text) => {
                    println!("<li>{}</li>", text);
                }

                GemTextTokenType::Link { link, text } => {
                    println!(
                        "<a href=\"{}\">{}</a>",
                        link,
                        text.as_ref().unwrap_or(&link.to_string())
                    );
                }

                GemTextTokenType::PrefText { text, alt } => {
                    println!(
                        "<pre class=\"{0}\" title=\"{0}\">\n{1}</pre>",
                        alt.as_ref().unwrap_or(&String::new()),
                        text
                    );
                }

                GemTextTokenType::Blockquote(text) => {
                    println!("<blockquote>{}</blockquote>", text);
                }

                GemTextTokenType::PlainText(text) => {
                    println!("<p>{}</p>", text);
                }

                GemTextTokenType::EmptyLine => {
                    println!("<br/>");
                }
            }
        }
    }
}

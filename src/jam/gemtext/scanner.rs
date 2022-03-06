#[allow(unused_imports)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum GemTextTokenType {
    PlainText(String), //Just plain text

    Link {
        // `=>` link
        link: String,         // URL of link
        text: Option<String>, //Text of link
    },

    Heading {
        // `#` Header
        level: usize, // Level of Heading => possible values => {1 , 2 ,3}
        text: String, // header text
    },

    List(String), // gemtext list `*`

    Blockquote(String), //gemtext Blockquote `>`

    PrefText {
        // gemtext preformatted text '```'
        text: String,        // text inside the preformatted block
        alt: Option<String>, // alttext of preformatted block
    },

    EmptyLine, // Just empty line
}

#[derive(Debug)]
pub struct GemTextToken {
    pub _type: GemTextTokenType,
    pub _line: usize,
}

fn new_token(_type: GemTextTokenType, _line: usize) -> GemTextToken {
    GemTextToken { _type, _line }
}

pub struct GemScanner {
    _source_vec: String,
    line: usize,
    item_list: Vec<GemTextToken>,
}

// Gemtext Parser

impl GemScanner {
    pub fn new(source: String) -> GemScanner {
        Self {
            _source_vec: source,
            line: 1,
            item_list: Vec::new(),
        }
    }

    pub fn get_tokens(self) -> Vec<GemTextToken> {
        self.item_list
    }

    pub fn print_tokens(&self) {
        println!("length => {}", &self.item_list.len());

        for item in &self.item_list {
            println!("{:?}", item);
        }
    }

    pub fn scan_tokens(&mut self) {
        let mut in_preftext: bool = false;
        let preftext_alt = &mut String::new();
        let preftext_text = &mut String::new();
        let mut _tok: GemTextToken;
        for _ln_ in self._source_vec.split('\n') {
            self.line += 1;
            let mut _line = String::from(_ln_);
            if !in_preftext {
                if let Some(stripped) = _line.trim_start().strip_prefix("### ") {
                    _tok = new_token(
                        GemTextTokenType::Heading {
                            level: 3,
                            text: stripped.trim().to_string(),
                        },
                        self.line,
                    )
                } else if let Some(stripped) = _line.trim_start().strip_prefix("## ") {
                    _tok = new_token(
                        GemTextTokenType::Heading {
                            level: 2,
                            text: stripped.trim().to_string(),
                        },
                        self.line,
                    )
                } else if let Some(stripped) = _line.trim_start().strip_prefix("# ") {
                    _tok = new_token(
                        GemTextTokenType::Heading {
                            level: 1,
                            text: stripped.trim().to_string(),
                        },
                        self.line,
                    )
                } else if let Some(stripped) = _line.trim_start().strip_prefix("* ") {
                    _tok = new_token(
                        GemTextTokenType::List(stripped.trim().to_string()),
                        self.line,
                    );
                } else if let Some(raw_preftext) = _line.trim_start().strip_prefix("```") {
                    in_preftext = true;
                    *preftext_alt = raw_preftext.to_string();
                    continue;
                } else if _line.is_empty() {
                    _tok = new_token(GemTextTokenType::EmptyLine, self.line);
                } else if let Some(text) = _line.trim_start().strip_prefix("> ") {
                    _tok = new_token(
                        GemTextTokenType::Blockquote(text.trim().to_string()),
                        self.line,
                    );
                } else if let Some(raw_link) = _line.trim_start().strip_prefix("=>") {
                    let _tmp_line: Vec<String> =
                        raw_link.trim().split(' ').map(str::to_string).collect();
                    let mut _label_ = String::new();
                    if _tmp_line.len() > 1 {
                        _label_ = _tmp_line[1..].join(" ").trim().to_string();
                    }
                    _tok = new_token(
                        GemTextTokenType::Link {
                            link: _tmp_line[0].to_string(),
                            text: if _label_.is_empty() {
                                None
                            } else {
                                Some(_label_)
                            },
                        },
                        self.line,
                    )
                } else {
                    _tok = new_token(
                        GemTextTokenType::PlainText(_line.trim().to_string()),
                        self.line,
                    );
                }

                self.item_list.push(_tok);
            } else if _line.trim_start().starts_with("```") {
                _tok = new_token(
                    GemTextTokenType::PrefText {
                        text: preftext_text.to_string(),
                        alt: if preftext_alt.is_empty() {
                            None
                        } else {
                            Some(preftext_alt.to_string())
                        },
                    },
                    self.line,
                );
                self.item_list.push(_tok);
                in_preftext = false;
                continue;
            } else {
                preftext_text.push_str(&_line);
                preftext_text.push('\n');
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    pub trait Anytype {
        fn type_name(&self) -> &'static str;
    }

    impl<T> Anytype for T {
        fn type_name(&self) -> &'static str {
            std::any::type_name::<T>()
        }
    }

    #[test]
    fn test_header_1() {
        let input = String::from("# header one");

        let mut test_scanner = GemScanner::new(input);
        test_scanner.scan_tokens();
        let tokens = test_scanner.get_tokens();
        //assert_eq!(tokens[0]._type. , );
        assert_eq!(tokens.len(), 1);
        //assert_eq!(, GemTextTokenType::Heading { level , text })
        //assert!()
    }
}

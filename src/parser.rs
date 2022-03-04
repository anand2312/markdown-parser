use crate::lexer::{Lexer, LexerToken};

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum ParsedToken {
    Header { level: usize, content: String },
    Bold { content: String },
    Italic { content: String },
    Link { text: String, href: String },
    Text { content: String },
}

#[derive(Debug)]
pub struct Block {
    // Blocks are different sections of the document,
    // separated by double-newlines (a blank line in betweem)
    pub children: Vec<ParsedToken>,
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.children == other.children
    }
}

#[allow(dead_code)]
pub struct Parser {
    pub lexer_tokens: Vec<LexerToken>,
    current: usize,
}

// leaf-type tag (only opens, closes at line break): header, list-item
// container tag (both opens and closes): bold, italic etc
// a header will be an entire Block
// miscellaneous content will be in a Block (which would be turned into a p tag)
// lists will be a Block

impl Parser {
    pub fn new(content: &str) -> Parser {
        let lexer = Lexer::new(content);
        Parser {
            lexer_tokens: lexer
                .into_iter()
                .filter(|x| !matches!(x, LexerToken::CarriageReturn))
                .collect(),
            current: 0,
        }
    }

    pub fn with_current_at(content: &str, current: usize) -> Parser {
        let lexer = Lexer::new(content);
        Parser {
            lexer_tokens: lexer
                .into_iter()
                .filter(|x| !matches!(x, LexerToken::CarriageReturn))
                .collect(),
            current,
        }
    }

    fn peak_next(&self, current: &usize) -> &LexerToken {
        if current >= &self.lexer_tokens.len() {
            return &LexerToken::EOF;
        }
        &self.lexer_tokens[*current]
    }

    fn peak_back(&self, current: &usize) -> &LexerToken {
        if current == &0 {
            return &LexerToken::EOF;
        }
        &self.lexer_tokens[*current]
    }

    pub fn parse_hashtag(&mut self) -> Option<Block> {
        // When a # is encountered, first check if it is on
        // it's own line (it follows a \n)
        // Then count it's level
        // and end it's content when another \n is encountered.
        let is_header = match self.peak_back(&self.current) {
            &LexerToken::NewLine => true,
            &LexerToken::EOF => true,
            _ => false,
        };

        if !is_header {
            // TODO: FIX!! the next content tag may be inside some other container which would mess up position.
            // add the # sign to a nearby Content tag
            let mut i = self.current.clone() + 1;
            loop {
                let next_token = &self.lexer_tokens[i];
                match next_token {
                    LexerToken::Content(s) => {
                        // replace the content with a new one that includes
                        // the # sign
                        let mut new_content = String::from("#");
                        new_content = new_content + s.as_str();
                        self.lexer_tokens[i] = LexerToken::Content(new_content);
                        break;
                    }
                    _ => {
                        i += 1;
                    }
                };
            }
            return None;
        } else {
            let mut level: usize = 1;
            let mut header_content = String::new();
            self.current += 1;

            loop {
                let next_token = self.peak_next(&self.current);

                match next_token {
                    LexerToken::Hash => {
                        level += 1;
                    }
                    LexerToken::Content(s) => {
                        header_content += s;
                    }
                    LexerToken::NewLine => {
                        break Some(Block {
                            children: vec![ParsedToken::Header {
                                level: level,
                                content: header_content,
                            }],
                        });
                    }
                    LexerToken::EOF => {
                        break Some(Block {
                            children: vec![ParsedToken::Header {
                                level: level,
                                content: header_content,
                            }],
                        });
                    }
                    _ => {}
                };
                self.current += 1;
            }
        }
    }
}

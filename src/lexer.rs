use std::str::Chars;

#[derive(Debug)]
pub enum LexerToken {
    Hash,
    Asterisk,
    Underscore,
    Backslash,
    OpenParen,
    CloseParen,
    OpenSquareBracket,
    CloseSquareBracket,
    CarriageReturn, // \r - appears on Windows paired with \n
    NewLine,
    EOF,
    Content(String),
}

pub struct Lexer<'a> {
    current: char,
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Lexer<'a> {
        let mut chars = content.chars();
        let current = chars.next().unwrap_or('\0');

        Lexer { current, chars }
    }

    fn advance_content(&mut self) -> String {
        // Keep advancing until a special identifier is reached.
        let mut to_yield = String::new();
        loop {
            match self.current {
                '\0' => break,
                '#' => break,
                '*' => break,
                '_' => break,
                '\\' => break,
                '(' => break,
                ')' => break,
                '[' => break,
                ']' => break,
                '\n' => break,
                '\r' => break,
                _ => {
                    to_yield.push(self.current);
                    self.current = self.chars.next().unwrap_or('\0');
                }
            }
        }
        to_yield
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexerToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == '\0' {
            // EOF
            return None;
        }

        let to_yield = match self.current {
            '#' => LexerToken::Hash,
            '*' => LexerToken::Asterisk,
            '_' => LexerToken::Underscore,
            '\\' => LexerToken::Backslash,
            '(' => LexerToken::OpenParen,
            ')' => LexerToken::CloseParen,
            '[' => LexerToken::OpenSquareBracket,
            ']' => LexerToken::CloseSquareBracket,
            '\r' => LexerToken::CarriageReturn,
            '\n' => LexerToken::NewLine,
            _ => LexerToken::Content(self.advance_content()),
        };

        match to_yield {
            // in case of plain content, Lexer::advance_content would have
            // advanced the chars iterator upto a special character
            // so we want to avoid advancing the iterator here again.
            LexerToken::Content(_) => {
                self.current = self.current;
            }
            _ => {
                self.current = self.chars.next().unwrap_or('\0');
            }
        }

        Some(to_yield)
    }
}

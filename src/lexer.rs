use std::str::Chars;

#[derive(Debug)]
pub enum MarkdownToken {
    Hash,
    Asterisk,
    Underscore,
    Backslash,
    OpenParen,
    CloseParen,
    OpenSquareBracket,
    CloseSquareBracket,
    Content(String),
}

#[derive(Debug)]
pub struct Position {
    line: usize,
    index: usize,
}

impl Position {
    pub fn next_pos(&self, next_line: bool) -> Position {
        if !next_line {
            Position {
                line: self.line + 1,
                index: 0,
            }
        } else {
            Position {
                line: self.line,
                index: self.index + 1,
            }
        }
    }
}

pub struct Lexer<'a> {
    current: char,
    current_pos: Position,
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Lexer<'a> {
        let mut chars = content.chars();
        let current = chars.next().unwrap_or('\0');
        let current_pos = Position { line: 1, index: 0 };

        Lexer {
            current,
            chars,
            current_pos,
        }
    }

    fn advance_content(&mut self) -> String {
        // Keep advancing until a special identifier is reached.
        // TODO: impl position tracker.
        let mut to_yield = String::new();
        loop {
            match self.current {
                '#' => break,
                '*' => break,
                '_' => break,
                '\\' => break,
                '(' => break,
                ')' => break,
                '[' => break,
                ']' => break,
                _ => to_yield.push(self.current),
            }
            self.current = self.chars.next().unwrap_or('\0');
        }
        to_yield
    }
    pub fn next(&mut self) -> Option<MarkdownToken> {
        // TODO: impl position tracker.
        if self.current == '\0' {
            // EOF
            return None;
        }

        let to_yield = match self.current {
            '#' => MarkdownToken::Hash,
            '*' => MarkdownToken::Asterisk,
            '_' => MarkdownToken::Underscore,
            '\\' => MarkdownToken::Backslash,
            '(' => MarkdownToken::OpenParen,
            ')' => MarkdownToken::CloseParen,
            '[' => MarkdownToken::OpenSquareBracket,
            ']' => MarkdownToken::CloseSquareBracket,
            _ => MarkdownToken::Content(self.advance_content()),
        };

        Some(to_yield)
    }
}

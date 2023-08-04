pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub enum Prefix {
    None,
    Whitespace,
    Title,
    OrderedList,
    UnorderedList,
    Number,
    NumberDot,
    Others,
}

#[derive(Clone, Debug)]
pub enum Tok {
    Space,
    Tab,
    Linefeed,
    Backtick,
    Hash,
    Hashes(u32),
    Plus,
    Minus,
    Star,
    Slash,
    Dot,
    LParen,
    RParen,
    Name(String),
    Number(i32),
    TitleName(String),
    QuotedItem(String),
}

#[derive(Debug)]
pub enum LexicalError {
    // Not possible
}

use std::{hash, str::CharIndices};
pub struct Lexer<'input> {
    chars: CharIndices<'input>,
    cur_ch: char,
    last_ch: char,
    cur_idx: usize,
    withdraw: bool,
    eof: bool,
    is_line_begin: bool,
    prefix: Prefix,
    line_number: u32,
}

type Item = Spanned<Tok, usize, LexicalError>;

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices(),
            cur_ch: '\n',
            last_ch: '\0',
            cur_idx: 0,
            withdraw: false,
            eof: false,
            is_line_begin: true,
            prefix: Prefix::None,
            line_number: 0,
        }
    }

    pub fn is_name_start(&self) -> bool {
        if self.eof {
            return false;
        }
        self.cur_ch.is_alphabetic() || self.cur_ch == '_'
    }

    pub fn is_number_start(&self) -> bool {
        if self.eof {
            return false;
        }
        self.cur_ch.is_digit(10)
    }

    pub fn is_name_continuation(&self) -> bool {
        if self.eof {
            return false;
        }
        self.is_name_start() || self.cur_ch.is_digit(10)
    }

    pub fn get_next_char(&mut self) -> Option<(usize, char)> {
        if self.withdraw {
            self.withdraw = false;
            return Some((self.cur_idx, self.cur_ch));
        }

        self.last_ch = self.cur_ch;

        let c = self.chars.next();
        if c.is_none() {
            self.eof = true;
            self.cur_idx += 1;
            self.cur_ch = '\0';
            return None; // EOF
        }

        let (i, ch) = c.unwrap();
        self.cur_ch = ch;
        self.cur_idx = i;
        Some((i, ch))
    }

    pub fn withdraw_last_ch(&mut self) {
        self.withdraw = true;
    }

    pub fn consume_hashes(&mut self) -> Option<Item> {
        let i = self.cur_idx;

        if matches!(self.prefix, Prefix::Whitespace | Prefix::None) {
            let mut hash_cnt: usize = 0;
            while self.cur_ch == '#' {
                hash_cnt += 1;
                self.get_next_char();
            }

            // `## title` is valid
            if self.cur_ch != ' ' {
                self.withdraw_last_ch();
                return None;
            }

            self.withdraw_last_ch();
            self.prefix = Prefix::Title;
            return Some(Ok((i, Tok::Hashes(hash_cnt as u32), i + hash_cnt)));
        }

        Some(Ok((i, Tok::Hash, i + 1)))
    }

    pub fn consume_special_char(&mut self, t: Tok) -> Option<Item> {
        self.prefix = Prefix::Others;

        let i = self.cur_idx;

        Some(Ok((i, t, i + 1)))
    }

    pub fn consume_number(&mut self) -> Option<Item> {
        let start_idx = self.cur_idx;
        let mut word = String::new();
        while self.is_number_start() {
            word.push(self.cur_ch);
            self.get_next_char();
        }
        self.withdraw_last_ch();
        Some(Ok((
            start_idx,
            Tok::Number(word.parse::<i32>().unwrap()),
            self.cur_idx,
        )))
    }

    pub fn consume_name(&mut self) -> Option<Item> {
        let start_idx = self.cur_idx;
        let mut word = String::new();

        while self.is_name_continuation() {
            word.push(self.cur_ch);
            self.get_next_char();
        }
        self.withdraw_last_ch();
        Some(Ok((start_idx, Tok::Name(word), self.cur_idx)))
    }

    pub fn consume_until(&mut self, end: char) -> String {
        let mut word = String::new();
        while self.cur_ch != end && !self.eof {
            word.push(self.cur_ch);
            self.get_next_char();
        }
        word
    }

    pub fn comsume_until_endl(&mut self) -> String {
        self.consume_until('\n')
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let c = self.get_next_char();
            if self.eof {
                return None; // EOF
            }

            // update state
            self.is_line_begin = self.last_ch == '\n';
            if self.is_line_begin {
                self.prefix = Prefix::None;
            }

            if matches!(self.prefix, Prefix::Title) {
                let i = self.cur_idx;
                let s = self.comsume_until_endl();
                let tok = Tok::TitleName(s.trim().to_string());
                return Some(Ok((i, tok, self.cur_idx)));
            }

            if self.is_name_start() {
                return self.consume_name();
            }

            if self.is_number_start() {
                if matches!(self.prefix, Prefix::None | Prefix::Whitespace) {
                    self.prefix = Prefix::Number;
                }
                return self.consume_number();
            }

            match c {
                Some((i, ' ')) => {
                    match self.prefix {
                        Prefix::None => self.prefix = Prefix::Whitespace,
                        Prefix::Number => self.prefix = Prefix::Others,
                        Prefix::NumberDot => self.prefix = Prefix::OrderedList,
                        _ => {}
                    }
                    return Some(Ok((i, Tok::Space, i + 1)));
                }
                Some((i, '\t')) => {
                    match self.prefix {
                        Prefix::None => self.prefix = Prefix::Whitespace,
                        Prefix::Number => self.prefix = Prefix::Others,
                        Prefix::NumberDot => self.prefix = Prefix::OrderedList,
                        _ => {}
                    }
                    return Some(Ok((i, Tok::Tab, i + 1)));
                }
                Some((i, '\n')) => {
                    self.line_number += 1;
                    return Some(Ok((i, Tok::Linefeed, i + 1)));
                }
                Some((i, '#')) => {
                    let t = self.consume_hashes();
                    if t.is_some() {
                        return t;
                    }
                    continue;
                }

                Some((i, '`')) => {
                    let mut s = String::new();
                    s.push(self.cur_ch);
                    self.get_next_char();
                    while self.cur_ch != '`' && self.cur_ch != '\n' && !self.eof {
                        s.push(self.cur_ch);
                        self.get_next_char();
                    }
                    if self.cur_ch == '`' {
                        s.push(self.cur_ch);
                        let n = s.len();
                        return Some(Ok((i, Tok::QuotedItem(s), i + n)));
                    } else {
                        continue;
                    }
                }
                Some((_, '+')) => return self.consume_special_char(Tok::Plus),
                Some((_, '-')) => return self.consume_special_char(Tok::Minus),
                Some((_, '*')) => return self.consume_special_char(Tok::Star),
                Some((_, '/')) => return self.consume_special_char(Tok::Slash),
                Some((_, '(')) => return self.consume_special_char(Tok::LParen),
                Some((_, ')')) => return self.consume_special_char(Tok::RParen),
                Some((i, '.')) => {
                    if matches!(self.prefix, Prefix::Number) {
                        self.prefix = Prefix::NumberDot;
                    }
                    return Some(Ok((i, Tok::Dot, i + 1)));
                }

                _ => continue,
            }
        }
    }
}

#[test]
fn skip_comments() {
    let source = "# The 12. quick brown fox jumped over the lazy `dog`.";
    for ele in Lexer::new(source).collect::<Vec<_>>().iter() {
        println!("{:?}", ele);
    }
}

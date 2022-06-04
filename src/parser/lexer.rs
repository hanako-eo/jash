use std::process;

use super::{Token, TokenKind};

pub struct Lexer {
  content: Vec<char>,
  c: char,
  i: usize,

  current_column: usize,
  current_line: usize
}

impl Lexer {
  pub fn init(contents: String) -> Self {
    let mut content: Vec<char> = contents.chars().collect();
    content.push('\0');

    return Self {
      content: content.clone(),
      c: content[0],
      i: 0,

      current_column: 0,
      current_line: 0
    }
  }

  pub fn skip_whitespace(&mut self) {
    while self.c.is_whitespace() {
      if self.c == '\n' {
        self.current_line += 1;
        self.current_column = 0;
      }
      self.skip(1);
    }
  }

  pub fn next_c(&mut self) -> Option<char> {
    self.i += 1;
    if self.c != '\0' && self.i < self.content.len() {
      self.current_column += 1;
      self.c = self.content[self.i];
      return Some(self.c)
    }

    None
  }

  pub fn skip(&mut self, gap: u8) -> bool {
    for _ in 0..gap {
      match self.next_c() {
        Some(_) => continue,
        None => return false
      }
    }
    true
  }

  pub fn peek(&self, offset: usize) -> String {
    let i = self.i + 1;
    let chars = self.content[i..std::cmp::min(i + offset, self.content.len())].into_iter();
    String::from_iter(chars)
  }

  pub fn skip_with_token(&mut self, gap: u8, token: Token) -> Token {
    self.skip(gap + 1);
    token
  }

  pub fn is_id_valid(&self) -> bool {
    !self.c.is_whitespace() && self.c != '"' && self.c != '\'' && self.c != '`' && self.c != '#'
  }

  pub fn collect_id(&mut self) -> Token {
    let mut value = String::new();
    let column = self.current_column;
    let line = self.current_line;

    while self.is_id_valid() && self.c != '\0' {
      value += &self.c.to_string();
      if !self.skip(1) {
        break
      }
    }

    Token::init(TokenKind::ID(value.clone()), line, column, value.len())
  }

  pub fn collect_comment(&mut self) -> Token {
    let mut value = String::new();
    let column = self.current_column;
    let line = self.current_line;
    self.skip(1);

    while self.c != '\n' && self.c != '\0' {
      value += &self.c.to_string();
      if !self.skip(1) {
        break
      }
    }

    let value = String::from(value.trim());
    Token::init(TokenKind::Comment(value.clone()), line, column, value.len())
  }

  pub fn collect_string(&mut self, c: char) -> Token {
    let mut value = String::new();
    let column = self.current_column;
    let line = self.current_line;
    self.skip(1);

    while self.c != c || value.ends_with("\\") {
      value += &self.c.to_string();
      if !self.skip(1) {
        break
      }
    }
    self.skip(1);

    Token::init(TokenKind::String(value.clone()), line, column, value.len())
  }
}

impl Iterator for Lexer {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    while self.c != '\0' && self.i < self.content.len() {
      if self.c.is_whitespace() {
        self.skip_whitespace();
      }

      if self.c == '"' || self.c == '\'' || self.c == '`' {
        return Some(self.collect_string(self.c))
      }

      let peek = self.peek(1);

      let column = self.current_column;
      let line = self.current_line;
      match self.c {
        '#' => return Some(self.collect_comment()),
        ';' => return Some(self.skip_with_token(1, Token::init(TokenKind::Then, line, column, 1))),
        '|' => {
          if peek == "|" {
            return Some(self.skip_with_token(1, Token::init(TokenKind::OR, line, column, 2)))
          }

          process::exit(1);
        },
        '&' => {
          if peek == "&" {
            return Some(self.skip_with_token(1, Token::init(TokenKind::AND, line, column, 2)))
          }
          return Some(self.skip_with_token(1, Token::init(TokenKind::Ampersand, line, column, 1)))
        },
        '\0' => return None,
        _ => return Some(self.collect_id())
      }
    }
    None
  }
}

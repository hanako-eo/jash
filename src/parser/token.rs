#[derive(Debug, PartialEq)]
pub enum TokenKind {
  ID(String),
  String(String),  // ""
  Comment(String), // #
  AND,             // &&
  OR,              // ||
  Then,            // ;
  Ampersand        // &
}

#[derive(Debug)]
pub struct Token {
  pub kind: TokenKind,
  pub line: usize,
  pub column: usize,
  pub size: usize
}

impl Token {
  pub fn init<'a>(kind: TokenKind, line: usize, column: usize, size: usize) -> Self {
    Token {
      kind,
      line,
      column,
      size
    }
  }
}

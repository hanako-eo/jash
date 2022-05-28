use crate::io::which;
use crate::parser::{Lexer, TokenKind};

#[derive(Debug, Default)]
pub struct Args {
  pub path: Option<String>,
  pub program: String,
  pub args: Vec<String>
}

impl Args {
  pub fn from_str(command: String) -> Args {
    let mut args = Args::default();
    for token in Lexer::init(command) {
      let value = match token.kind {
        TokenKind::ID(value) | TokenKind::String(value) => value,
        _ => String::new()
      };
      if args.program.is_empty() {
        args.path = which(&value);
        args.program = value;
      } else {
        args.args.push(value);
      }
    }
    args
  }
}

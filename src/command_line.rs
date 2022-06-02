use crate::io::which;
use crate::parser::{Lexer, TokenKind};

#[derive(Debug, Default)]
pub struct CommandLine {
  pub path: Option<String>,
  pub command: String,
  pub args: Vec<String>
}

impl CommandLine {
  pub fn from_str(command: String) -> CommandLine {
    let mut args = CommandLine::default();
    for token in Lexer::init(command) {
      let value = match token.kind {
        TokenKind::ID(value) | TokenKind::String(value) => value,
        _ => String::new()
      };
      if args.command.is_empty() {
        args.path = which(&value);
        args.command = value;
      } else {
        args.args.push(value);
      }
    }
    args
  }
}

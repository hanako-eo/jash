use crate::io::which;
use crate::parser::{Lexer, TokenKind};

#[derive(Debug, Default)]
pub enum CommandModifier {
  And(Box<CommandLine>),
  Or(Box<CommandLine>),
  
  #[default]
  None
}

#[derive(Debug, Default)]
pub struct CommandLine {
  command: String,
  args: Vec<String>,
  modifier: CommandModifier
}

impl CommandLine {
  pub fn new(command: String) -> Self {
    let mut iter = Lexer::init(command);
    Self::from_lexer(&mut iter)
  }
  
  fn from_lexer(iter: &mut Lexer) -> Self {
    let mut command_line = CommandLine::default();
    while let Some(token) = iter.next() {
      match token.kind {
        TokenKind::ID(value) | TokenKind::String(value) => {
          if command_line.command.is_empty() {
            command_line.command = value;
          } else {
            command_line.args.push(value);
          }
        },
        TokenKind::AND => {
          command_line.modifier = CommandModifier::And(Box::new(Self::from_lexer(iter)));
        },
        TokenKind::OR => {
          command_line.modifier = CommandModifier::Or(Box::new(Self::from_lexer(iter)));
        }
        _ =>()
      }
    }
    command_line
  }
  
  pub fn path(&self) -> Option<String> {
    which(&self.command)
  }

  pub fn args(&self) -> &Vec<String> {
    &self.args
  }
  pub fn command(&self) -> &String {
    &self.command
  }
  pub fn modifier(&self) -> &CommandModifier {
    &self.modifier
  }
}

mod builtin;
mod command_line;
mod env;
mod io;
mod parser;

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use builtin::{BuiltIn, Exit, CD, PWD};
use command_line::{CommandLine, CommandModifier};
use env::{system, vars};

#[derive(Debug)]
struct App {
  built_ins: HashMap<&'static str, Box<dyn BuiltIn>>
}

impl App {
  fn new() -> Self {
    vars::create("PS1", "\x1b[1;34m\\w\x1b[0m $\n> ");

    let mut built_ins: HashMap<&str, Box<dyn BuiltIn>> = HashMap::new();

    built_ins.insert("cd", Box::new(CD::new()));
    built_ins.insert("exit", Box::new(Exit::new()));
    built_ins.insert("pwd", Box::new(PWD::new()));

    Self {
      built_ins
    }
  }

  fn ps1(&self) -> String {
    let home = vars::get("HOME");
    let cwd = vars::get("PWD");

    let pwd = if cwd.starts_with(&home) {
      cwd.replacen(&home, "~", 1)
    } else {
      cwd.clone()
    };

    let work_path = PathBuf::from(&cwd);
    let current_folder = match work_path.file_name() {
      Some(name) => name.to_str().unwrap(),
      None => "/"
    };

    vars::get("PS1")
      .replace(r"\h", &system::get_hostname())
      .replace(r"\u", &system::get_user())
      .replace(r"\y", &cwd)
      .replace(r"\Y", &current_folder)
      .replace(r"\w", &pwd)
      .replace(r"\W", if &pwd == "~" { &"~" } else { &current_folder })
  }

  fn execute(&mut self, command_line: &CommandLine) -> i8 {
    if let Some(path) = command_line.path() {
      return self.exec_process(command_line, path)
    } else {
      for (command, built_in) in &mut self.built_ins {
        if command == &command_line.command() {
          return built_in.handler(command_line)
        }
      }
    }
    127
  }
  
  fn exec_process(&mut self, command_line: &CommandLine, path: String) -> i8 {
    let r = Command::new(path)
      .args(command_line.args())
      .env_clear()
      .envs(vars::all())
      .current_dir(vars::get("PWD"))
      .spawn();
  
    if let Ok(mut process) = r {
      return match process.wait() {
        Ok(status) =>{
          let code = status.code().unwrap_or(1) as i8;
          match command_line.modifier() {
            CommandModifier::And(next_command) => {
              return if code == 0 {
                 self.execute(next_command.as_ref())
              } else {
                code
              }
            },
            CommandModifier::Or(next_command) => {
              return if code > 0 {
                 self.execute(next_command.as_ref())
              } else {
                code
              }
            },
            CommandModifier::None => code
          }
          
        },
        Err(_) => 127
      }
    }
    0
  }
}

fn main() {
  let mut app = App::new();

  loop {
    let value = input!("{}", app.ps1());
    let command_line = value.trim().to_string();
    if command_line.is_empty() {
      continue
    }

    let code = app.execute(&CommandLine::new(command_line));
    vars::set("?", code.to_string());
  }
}

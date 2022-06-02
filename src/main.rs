mod args;
mod builtin;
mod env;
mod io;
mod parser;

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use args::Args;
use builtin::{BuiltIn, CD, Exit};
use env::{system, vars};

#[derive(Debug)]
struct App {
  work_dir: PathBuf,
  built_ins: HashMap<&'static str, Box<dyn BuiltIn>>
}

impl App {
  fn new() -> Self {
    vars::create("PS1", "\x1b[1;34m\\w\x1b[0m $\n> ");

    let mut built_ins: HashMap<&str, Box<dyn BuiltIn>> = HashMap::new();
    let work_dir = PathBuf::from(&vars::get_result("PWD").unwrap_or("/".to_string()));

    built_ins.insert("cd", Box::new(CD::new()));
    built_ins.insert("exit", Box::new(Exit::new()));

    Self {
      built_ins,
      work_dir
    }
  }

  fn ps1(&self) -> String {
    let home = vars::get("HOME");
    let cwd = self.work_dir.to_str().unwrap_or(&home);

    let pwd = if cwd.starts_with(&home) {
      cwd.replacen(&home, "~", 1)
    } else {
      cwd.to_string()
    };

    let current_folder = match self.work_dir.file_name() {
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

  fn execute(&mut self, command_line: Args) -> i8 {
    if let Some(command_path) = command_line.path {
      let r = Command::new(command_path)
        .args(&command_line.args)
        .env_clear()
        .envs(vars::all())
        .current_dir(&self.work_dir)
        .spawn();

      if let Ok(mut process) = r {
        return match process.wait() {
          Ok(status) => status.code().unwrap_or(1) as i8,
          Err(_) => 127
        }
      }
    } else {
      for (command, built_in) in &self.built_ins {
        if command == &command_line.program {
          return built_in.handler(&mut self.work_dir, &command_line.args)
        }
      }
    }
    127
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

    let code = app.execute(Args::from_str(command_line));
    vars::set("?", code.to_string());
  }
}

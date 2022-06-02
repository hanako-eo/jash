use std::path::PathBuf;

use crate::command_line::CommandLine;
use crate::env::vars;

use super::BuiltIn;

#[derive(Debug)]
pub struct CD;

impl CD {
  pub fn new() -> Self {
    Self {}
  }
}

impl BuiltIn for CD {
  fn handler(&mut self, command_line: CommandLine) -> i8 {
    let home = vars::get("HOME");
    let mut work_dir = PathBuf::from(vars::get_result("PWD").unwrap_or(home.clone()));
    let prev_work_dir = vars::get_result("OLDPWD").ok();
    if let Some(dest) = command_line.args.get(0) {
      let dest = if dest.starts_with("~") {
        dest.replacen("~", &home, 1)
      } else {
        dest.to_string()
      };

      if dest.as_str() == "-" {
        if let Some(prev_work_dir) = prev_work_dir {
          work_dir.clear();
          work_dir.push(&prev_work_dir);
          println!("{}", &prev_work_dir);
        } else {
          eprintln!("cd: OLDPWD not set");
        }
      } else {
        if dest.starts_with("/") {
          work_dir.clear();
        }
        work_dir.push(dest);
      }
    } else if command_line.args.len() == 0 {
      work_dir.clear();
      work_dir.push(&home);
    } else {
      eprintln!("cd: too many args");
      return 1
    }
    match work_dir.canonicalize() {
      Ok(nwd) => {
        vars::set("OLDPWD", work_dir.to_str().unwrap());
        vars::set("PWD", nwd.to_str().unwrap_or(&home));
        0
      },
      Err(e) => {
        eprintln!("{}", e);
        1
      }
    }
  }
}

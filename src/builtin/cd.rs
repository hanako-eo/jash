use std::path::PathBuf;

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
  fn handler(&self, work_dir: &mut PathBuf, args: &Vec<String>) -> i8 {
    let prev_work_dir = vars::get_result("OLDPWD").ok();
    let home = vars::get("HOME");
    vars::set("OLDPWD", work_dir.to_str().unwrap());
    if let Some(dest) = args.get(0) {
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
    } else if args.len() == 0 {
      work_dir.clear();
      work_dir.push(&home);
    } else {
      eprintln!("cd: too many args");
      return 1
    }
    *work_dir = match work_dir.canonicalize() {
      Ok(nwd) => nwd,
      Err(e) => {
        eprintln!("{}", e);
        return 1
      }
    };
    0
  }
}

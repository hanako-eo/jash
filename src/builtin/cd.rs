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
    let prev_work_dir = vars::get("OLDPWD");
    let home = vars::get("HOME");
    vars::set("OLDPWD", work_dir.to_str().unwrap());
    if let Some(dest) = args.get(0) {
      let dest = if dest.starts_with("~") {
        dest.replacen("~", &home, 1)
      } else {
        dest.to_string()
      };

      if dest.as_str() == "-" {
        work_dir.clear();
        work_dir.push(&prev_work_dir);
        println!("{}", &prev_work_dir)
      } else {
        if dest.starts_with("/") {
          work_dir.clear();
        }
        work_dir.push(dest);
      }
    } else {
      work_dir.clear();
      work_dir.push(&home);
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

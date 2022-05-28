mod env;
mod io;

use std::path::PathBuf;

use colored::Colorize;
use env::Variables;

fn main() {
  let mut vars = Variables::new();
  let pwd_path = PathBuf::from(vars.get("PWD".to_string()).unwrap_or(&"/".to_string()));

  vars.set_local("PS1".to_string(), "\x1b[1;34m\\w\x1b[0m $\n> ".to_string());

  let hn = hostname::get().unwrap();
  let hn = hn.to_str().unwrap_or("pc");

  let username = users::get_current_username().unwrap();
  let username = username.to_str().unwrap();

  let home = vars.get("HOME".to_string()).unwrap();

  loop {
    let pwd = pwd_path.to_str().unwrap();
    let pwd = if pwd.starts_with(home) {
      pwd.replacen(home, "~", 1)
    } else {
      pwd.to_string()
    };

    let current_folder = pwd_path.file_name().unwrap().to_str().unwrap();
    let raw_ps1 = vars.get("PS1".to_string()).unwrap();
    let ps1 = raw_ps1
      .replace(r"\u", username)
      .replace(r"\h", hn)
      .replace(r"\w", &pwd)
      .replace(r"\W", current_folder);

    let result = input!("{}", ps1);
    println!("{}", result);
  }
}

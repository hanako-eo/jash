mod io;

use std::env;

use colored::Colorize;

fn main() {
  let pdw_path = env::current_dir().unwrap();
  let pwd = pdw_path.to_str().unwrap();
  loop {
    let PS1 = format!("{} $\n> ", pwd.blue());
    let result = input!("{}", PS1);
    println!("{}", result);
  }
}

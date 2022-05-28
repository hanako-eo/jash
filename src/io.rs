#[macro_export]
macro_rules! input {
  () => {
    input!("");
  };
  ($($arg:tt)*) => {{
    use std::io::Write;

    let input = std::io::stdin();
    let mut output = std::io::stdout();

    let _ = output.write_all(format!($($arg)*).as_bytes());
    let _ = output.flush();

    let mut buffer = String::new();
    let _ = input.read_line(&mut buffer);
    buffer
  }};
}

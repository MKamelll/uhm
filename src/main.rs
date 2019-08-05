//use std::process::Command;
#[macro_use]
mod parser;

fn main() {
  // let output = Command::new("git")
  //   .arg("status")
  //   .output()
  //   .expect("Could not excute");
  // println!("{}", String::from_utf8_lossy(&output.stdout));
  // println!("Hello, world!");
  parser::parse_args();
}

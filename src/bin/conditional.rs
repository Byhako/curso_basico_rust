use std::io::{self, Write};

fn main() {
  print!("\nTu edad: ");
  io::stdout().flush().unwrap();

  let mut age: String = String::new();
  io::stdin().read_line(&mut age).unwrap();

  let age_num: i8 = age.trim().parse().unwrap();

  if age_num <= 18 {
    println!("\nEres menor de edad");
  } else {
    println!("\nEres mayor de edad");
  }
}

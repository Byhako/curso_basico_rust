use std::io::{self, Write};

fn main() {
  print!("\n¿Hasta que número quieres contar?: ");
  io::stdout().flush().unwrap();

  let mut count: String = String::new();
  io::stdin().read_line(&mut count).unwrap();

  let limit: i8 = count.trim().parse().unwrap();

  let mut count_num = 0;

  println!("\nContando con loop:");

  loop {
    if count_num == limit {
      break;
    }

    println!("{}", count_num);
    count_num += 1;
  }

  println!("\nContando con while");

  count_num = 0;

  while count_num < limit {
    println!("{}", count_num);
    count_num += 1;
  }

  println!("\nContando con for");

  for i in 0..=limit {
    println!("{}", i);
  }

  println!("\nContando al reves");

  for i in (0..limit).rev() {
    println!("{}", i);
  }

  println!("¡Fin!");
}

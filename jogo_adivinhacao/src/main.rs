extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Adivinhe o número!");

  let numero_secreto = rand::thread_rng().gen_range(1..100);

  loop {
    println!("Digite um número (1 - 100):");

    let mut palpite = String::new();

    io::stdin().read_line(&mut palpite)
      .expect("Falha ao ler a entrada!");

    let palpite: u32 = palpite.trim().parse()
      .expect("Digite um número!");

    match palpite.cmp(&numero_secreto) {
      Ordering::Less => println!("Muito baixo!"),
      Ordering::Greater => println!("Muito alto!"),
      Ordering::Equal => {
        println!("ACERTOOOOOOOOOU!");
        break;
      }
    }
  }
}

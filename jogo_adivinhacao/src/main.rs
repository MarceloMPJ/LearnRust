extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Adivinhe o número! (Modo SUPER DIFICIL!)");

  let numero_secreto = rand::thread_rng().gen_range(1..100);

  println!("O número secreto é: {}", numero_secreto);

  println!("Nossa, qual será o número secreto??? Digite um número:");

  let mut palpite = String::new();

  io::stdin().read_line(&mut palpite)
    .expect("Falha ao ler a entrada!");

  let palpite: u32 = palpite.trim().parse()
    .expect("Digite um número!");

  match palpite.cmp(&numero_secreto) {
    Ordering::Less => println!("Muito baixo!"),
    Ordering::Greater => println!("Muito alto!"),
    Ordering::Equal => println!("ACERTOOOOOOOOOU!")
  }
}

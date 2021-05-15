#[derive(Debug)]
struct Retangulo {
  largura: u32,
  altura: u32,
}

impl Retangulo {
  fn area(&self) -> u32 {
    self.largura * self.altura
  }
}

fn main() {
  let retangulo = Retangulo { largura: 50, altura: 30 };

  println!("{:#?}", retangulo);

  println!("A área do retangulo é {} (pixels).",
    retangulo.area()
  )
}

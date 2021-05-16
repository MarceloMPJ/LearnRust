pub mod cliente;
pub mod network;

#[cfg(test)]
mod tests {
  use super::cliente;

  #[test]
  fn it_works() {
    cliente::conectar();
  }
}

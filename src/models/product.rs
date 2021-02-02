#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_product_should_have_name() {
      let prod = Product::new();

      assert_eq!("", prod.name());
  }
}
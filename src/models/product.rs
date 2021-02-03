use super::Named;
use super::inner_names::InnerName;

#[allow(dead_code)]
pub struct Product {
  inner_names: InnerName,
  v_code: String,
  bar_code: String,
}

impl Named for Product {
  fn name(&self) -> String {
    self.inner_names.name()
  }

  fn set_name(&mut self, value: &str) -> &Product {
    self.inner_names.set_name(value);
    self
  }

  fn description(&self) -> String {
    self.inner_names.description()
  }

  fn set_description(&mut self, value: &str) -> &Product {
    self.inner_names.set_description(value);
    self
  }
}

#[allow(dead_code)]
impl Product {
  pub fn new() -> Product {
    Product {
      inner_names: InnerName::new(),
      v_code: String::new(),
      bar_code: String::new(),
    }
  }

  pub fn v_code(&self) -> &str {
    &self.v_code
  }

  pub fn set_v_code(&mut self, value: &str) -> &Product {
    self.v_code = String::from(value);
    self
  }

  pub fn bar_code(&self) -> &str {
    &self.bar_code
  }

  pub fn set_bar_code(&mut self, value: &str) -> &Product {
    self.bar_code = String::from(value);
    self
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_product_should_have_empty_name() {
    let prod = Product::new();

    assert_eq!("", prod.name());
  }

  #[test]
  fn name_setter_should_change_product_instance() {
    assert_eq!("new", Product::new().set_name("new").name());
  }

  #[test]
  fn new_product_should_have_empty_description() {
    let prod = Product::new();

    assert_eq!("", prod.description());
  }

  #[test]
  fn description_setter_should_change_product_instance() {
    assert_eq!("new description", Product::new().set_description("new description").description());
  }

  #[test]
  fn new_product_should_have_empty_v_code() {
    let prod = Product::new();

    assert_eq!("", prod.v_code());
  }

  #[test]
  fn v_code_setter_should_change_product_instance() {
    assert_eq!("123456", Product::new().set_v_code("123456").v_code());
  }

  #[test]
  fn new_product_should_have_empty_bar_code() {
    let prod = Product::new();

    assert_eq!("", prod.bar_code());
  }

  #[test]
  fn bar_code_setter_should_change_product_instance() {
    assert_eq!("98765", Product::new().set_bar_code("98765").bar_code());
  }
}
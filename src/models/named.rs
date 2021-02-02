pub trait Named {
  fn name(&self) -> &str;
  fn set_name(&mut self, value: &str) -> &Self;
  fn description(&self) -> &str;
  fn set_description(&mut self, value: &str) -> &Self;
}
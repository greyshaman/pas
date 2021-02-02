pub trait Named {
  fn name(&self) -> String;
  fn set_name(&mut self, value: &str) -> &Self;
  fn description(&self) -> String;
  fn set_description(&mut self, value: &str) -> &Self;
}
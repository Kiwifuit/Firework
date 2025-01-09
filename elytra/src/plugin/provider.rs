pub trait ServerProvider {
  fn id(&self) -> &str;
  fn name(&self) -> &str;
  fn version(&self) -> &str;
}

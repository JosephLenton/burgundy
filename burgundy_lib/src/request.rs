
pub trait Request {
  fn to_url_path(&self) -> String;
}

use super::{module, util};

#[derive(Debug, Clone, PartialEq)]
pub struct Struct<T> {
  pub header: Header,
  pub module: module::Module<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Header {
  pub name: String,
  // pub type_parameters: Vec<util::TypeParameter>,
  pub traits: Vec<util::Path>,
}

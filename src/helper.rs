use super::node::*;

pub trait Helper<T>: Sized {
  fn eval(node: &RNode<T, Self>);
}

#[derive(Debug, Clone)]
pub struct DefaultHelper;
impl<T> Helper<T> for DefaultHelper {
  fn eval(_: &RNode<T, Self>) {}
}
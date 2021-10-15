use super::node::*;

pub trait Helper<T>: Sized {
  fn eval(node: &mut Node<Self, T>);
}
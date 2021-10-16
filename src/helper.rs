use std::{cell::RefCell, rc::Rc};

use super::node::*;

pub trait Helper<T>: Sized {
  fn eval(node: &Rc<RefCell<Node<Self, T>>>);
}
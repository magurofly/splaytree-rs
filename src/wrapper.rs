use super::{node::*, helper::*};

use std::rc::*;
use std::cell::*;

pub struct Wrapper<H, T> {
  node: Rc<RefCell<Node<H, T>>>,
}

impl<H: Helper<T>, T> Wrapper<H, T> {
  pub fn child(&self, dir: usize) -> Option<Self> {
    Node::child(&self.node, dir).map(Self::from)
  }

  pub fn parent(&self) -> Option<Self> {
    Node::parent(&self.node).map(Self::from)
  }

  pub fn set_child(&self, dir: usize, child: Option<Self>) {
    Node::set_child(&Parent::of(&self.node, dir), child.map(|c| c.node));
  }

  pub fn is_root(&self) -> bool {
    Node::is_root(&self.node)
  }

  pub fn value(&self) -> &T {
    Node::value(&self.node)
  }

  pub fn value_mut(&self) -> &mut T {
    Node::value_mut(&self.node)
  }
}

impl<H, T> From<Rc<RefCell<Node<H, T>>>> for Wrapper<H, T> {
  fn from(node: Rc<RefCell<Node<H, T>>>) -> Self {
    Self {
      node,
    }
  }
}
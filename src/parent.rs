use std::{rc::*};
use super::{node::*, helper::*, wrapper::*};

#[derive(Debug)]
/// 子から親へのポインタ
pub enum Parent<T, H> {
  L(WNode<T, H>),
  R(WNode<T, H>),
}

use Parent::*;

impl<T, H> Parent<T, H> {
  pub fn left(parent: &RNode<T, H>) -> Self {
    Self::L(Rc::downgrade(parent))
  }
  
  pub fn right(parent: &RNode<T, H>) -> Self {
    Self::R(Rc::downgrade(parent))
  }
  
  pub fn of(parent: &RNode<T, H>, dir: usize) -> Self {
    match dir {
      0 => Self::left(parent),
      1 => Self::right(parent),
      _ => unreachable!(),
    }
  }
  
  pub fn get(&self) -> RNode<T, H> {
    match self {
      L(parent) => Weak::upgrade(parent),
      R(parent) => Weak::upgrade(parent),
    }.unwrap()
  }
  
  pub fn dir(&self) -> usize {
    match self {
      L(_) => 0,
      R(_) => 1,
    }
  }
}

impl<T, H> Clone for Parent<T, H> {
  fn clone(&self) -> Self {
    match self {
      L(parent) => Self::L(parent.clone()),
      R(parent) => Self::R(parent.clone()),
    }
  }
}

impl<T, H: Helper<T>> Wrapper<T, H> for Parent<T, H> {
  /// unsafe
  fn node(&self) -> &Node<T, H> {
    unsafe { &*self.get().as_ptr() }
  }

  /// unsafe
  fn node_mut(&self) -> &mut Node<T, H> {
    unsafe { &mut *self.get().as_ptr() }
  }

  /// unsafe
  fn to_ref(&self) -> &RNode<T, H> {
    unsafe { std::mem::transmute::<_, &RNode<T, H>>(&self.get()) }
  }

  fn as_rnode(&self) -> RNode<T, H> {
    self.get()
  }
}
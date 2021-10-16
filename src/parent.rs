use std::{rc::*};
use super::{node::*, helper::*, wrapper::*};

#[derive(Debug)]
/// 子から親へのポインタ
pub enum Parent<H, T> {
  L(WNode<H, T>),
  R(WNode<H, T>),
}

use Parent::*;

impl<H, T> Parent<H, T> {
  pub fn left(parent: &RNode<H, T>) -> Self {
    Self::L(Rc::downgrade(parent))
  }
  
  pub fn right(parent: &RNode<H, T>) -> Self {
    Self::R(Rc::downgrade(parent))
  }
  
  pub fn of(parent: &RNode<H, T>, dir: usize) -> Self {
    match dir {
      0 => Self::left(parent),
      1 => Self::right(parent),
      _ => unreachable!(),
    }
  }
  
  pub fn get(&self) -> RNode<H, T> {
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

impl<H, T> Clone for Parent<H, T> {
  fn clone(&self) -> Self {
    match self {
      L(parent) => Self::L(parent.clone()),
      R(parent) => Self::R(parent.clone()),
    }
  }
}

impl<H: Helper<T>, T> Wrapper<H, T> for Parent<H, T> {
  /// unsafe
  fn node(&self) -> &Node<H, T> {
    unsafe { &*self.get().as_ptr() }
  }

  /// unsafe
  fn node_mut(&self) -> &mut Node<H, T> {
    unsafe { &mut *self.get().as_ptr() }
  }
}
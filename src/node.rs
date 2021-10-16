use crate::wrapper::Wrapper;

use super::{helper::*, parent::*};

use std::{cell::*, cmp::{Ordering::*, *}, marker::PhantomData, ops::Deref, rc::*};

pub type RNode<H, T> = Rc<RefCell<Node<H, T>>>;
pub type WNode<H, T> = Weak<RefCell<Node<H, T>>>;

#[derive(Debug, Default)]
pub struct Node<H, T> {
  parent: Option<Parent<H, T>>,
  children: [Option<RNode<H, T>>; 2],
  size: usize, depth: usize,
  value: T,
  phantom: PhantomData<H>,
}

impl<H: Helper<T>, T> Node<H, T> {
  pub fn new(value: T) -> Self {
    Self {
      parent: None,
      children: [None, None],
      size: 1, depth: 1,
      value,
      phantom: PhantomData,
    }
  }

  pub fn child(&self, dir: usize) -> Option<&RNode<H, T>> {
    self.children[dir].as_ref()
  }

  pub fn child_mut(&mut self, dir: usize) -> &mut Option<RNode<H, T>> {
    &mut self.children[dir]
  }

  pub fn parent(&self) -> Option<&Parent<H, T>> {
    self.parent.as_ref()
  }

  pub fn parent_mut(&mut self) -> &mut Option<Parent<H, T>> {
    &mut self.parent
  }

  pub fn size(&self) -> usize {
    self.size
  }

  pub fn size_mut(&mut self) -> &mut usize {
    &mut self.size
  }

  pub fn depth(&self) -> usize {
    self.depth
  }

  pub fn depth_mut(&mut self) -> &mut usize {
    &mut self.depth
  }

  pub fn value(&self) -> &T {
    &self.value
  }

  pub fn value_mut(&mut self) -> &mut T {
    &mut self.value
  }
}

impl<H, T: Clone> Clone for Node<H, T> {
  fn clone(&self) -> Self {
    Self {
      parent: self.parent.clone(),
      children: <[Option<Rc<_>>; 2]>::clone(&self.children),
      value: self.value.clone(),
      depth: self.depth,
      size: self.size,
      phantom: PhantomData,
    }
  }
}

impl<H: Helper<T>, T> Wrapper<H, T> for RNode<H, T> {
  /// unsafe
  fn node(&self) -> &Node<H, T> {
    unsafe { &*self.as_ptr() }
  }

  /// unsafe
  fn node_mut(&self) -> &mut Node<H, T> {
    unsafe { &mut *self.as_ptr() }
  }
}
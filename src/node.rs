use super::{helper::*, parent::*, wrapper::*};

use std::{cell::*, marker::PhantomData, rc::*};

pub type RNode<T, H> = Rc<RefCell<Node<T, H>>>;
pub type WNode<T, H> = Weak<RefCell<Node<T, H>>>;

#[derive(Default)]
pub struct Node<T, H = DefaultHelper> {
  parent: Option<Parent<T, H>>,
  children: [Option<RNode<T, H>>; 2],
  size: usize, depth: usize,
  value: T,
  phantom: PhantomData<H>,
}

impl<H, T> Node<T, H> {
  pub fn new(value: T) -> Self {
    Self {
      parent: None,
      children: [None, None],
      size: 1, depth: 1,
      value,
      phantom: PhantomData,
    }
  }

  pub fn child(&self, dir: usize) -> Option<&RNode<T, H>> {
    self.children[dir].as_ref()
  }

  pub fn child_mut(&mut self, dir: usize) -> &mut Option<RNode<T, H>> {
    &mut self.children[dir]
  }

  pub fn parent(&self) -> Option<&Parent<T, H>> {
    self.parent.as_ref()
  }

  pub fn parent_mut(&mut self) -> &mut Option<Parent<T, H>> {
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

impl<T: Clone, H> Clone for Node<T, H> {
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

impl<T, H: Helper<T>> Wrapper<T, H> for RNode<T, H> {
  /// unsafe
  fn node(&self) -> &Node<T, H> {
    unsafe { &*self.as_ptr() }
  }

  /// unsafe
  fn node_mut(&self) -> &mut Node<T, H> {
    unsafe { &mut *self.as_ptr() }
  }
}

impl<T: std::fmt::Debug, H> std::fmt::Debug for Node<T, H> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("Node(size={}, depth={}, value={:?}) [", self.size(), self.depth(), self.value()))?;
    if let Some(child) = self.child(0) {
      child.borrow().fmt(f)?;
    } else {
      f.write_str("Nil")?;
    }
    f.write_str(", ")?;
    if let Some(child) = self.child(1) {
      child.borrow().fmt(f)?;
    } else {
      f.write_str("Nil")?;
    }
    f.write_str("]")?;
    Ok(())
  }
}
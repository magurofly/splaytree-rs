use super::{helper::*};

use std::{marker::PhantomData, rc::*, cell::*};

type RNode<H, T> = Rc<RefCell<Node<H, T>>>;
type WNode<H, T> = Weak<RefCell<Node<H, T>>>;

#[derive(Debug, Default)]
pub struct Node<H, T> {
    parent: Option<Parent<H, T>>,
    children: [Option<RNode<H, T>>; 2],
    value: T,
    phantom: PhantomData<H>,
}

impl<H: Helper<T>, T> Node<H, T> {
    pub fn new(value: T) -> Self {
        Self {
            parent: None,
            children: [None, None],
            value,
            phantom: PhantomData,
        }
    }

    /// Warning: ref drops when this drops
    pub fn value(this: &RNode<H, T>) -> &T {
      &unsafe { &*RefCell::as_ptr(this) }.value
    }

    /// Warning: ref drops when this drops
    pub fn value_mut(this: &RNode<H, T>) -> &mut T {
      &mut unsafe { &mut *RefCell::as_ptr(this) }.value
    }

    pub fn is_root(this: &RNode<H, T>) -> bool {
      this.borrow().parent.is_none()
    }

    pub fn parent(this: &RNode<H, T>) -> Option<RNode<H, T>> {
      this.borrow().parent.as_ref().map(|p| p.node())
    }

    pub fn child(this: &RNode<H, T>, dir: usize) -> Option<RNode<H, T>> {
      this.borrow().children[dir].as_ref().map(Rc::clone)
    }

    pub fn set_parent(this: &RNode<H, T>, parent: Option<Parent<H, T>>) {
        if let Some(parent) = &parent {
            parent.set_child(Some(this.clone()));
        }
        this.borrow_mut().parent = parent;
    }

    pub fn set_child(parent: &Parent<H, T>, child: Option<RNode<H, T>>) {
        if let Some(child) = &child {
            child.borrow_mut().parent = Some(parent.clone());
        }
        parent.set_child(child);
    }

    pub fn rotate(this: &RNode<H, T>, dir: usize) -> RNode<H, T> {
        assert!(dir < 2);
        let child = this.borrow_mut().children[1 ^ dir].take().unwrap();
        Self::set_parent(&child, this.borrow_mut().parent.take());
        Self::set_child(&Parent::of(this, 1 ^ dir), child.borrow_mut().children[dir].take());
        Self::set_child(&Parent::of(&child, dir), Some(this.clone()));
        child
    }

    pub fn zig(this: &RNode<H, T>, dir: usize) {
      Self::rotate(this, dir);
    }

    pub fn zig_zig(this: &RNode<H, T>, dir: usize) {
      Self::rotate(&Self::rotate(this, dir), dir);
    }

    pub fn zig_zag(this: &RNode<H, T>, dir: usize) {
      Self::rotate(this.borrow().children[1 ^ dir].as_ref().unwrap(), 1 ^ dir);
      Self::rotate(this, dir);
    }

    pub fn splay(this: &RNode<H, T>) {
      todo!()
    }
}

impl<H, T: Clone> Clone for Node<H, T> {
    fn clone(&self) -> Self {
        Self {
            parent: self.parent.clone(),
            children: <[Option<Rc<_>>; 2]>::clone(&self.children),
            value: self.value.clone(),
            phantom: PhantomData,
        }
    }
}


#[derive(Debug)]
pub enum Parent<H, T> {
    L(WNode<H, T>),
    R(WNode<H, T>),
}

use Parent::*;

impl<H: Helper<T>, T> Parent<H, T> {
    pub fn l(parent: &RNode<H, T>) -> Self {
        Self::L(Rc::downgrade(parent))
    }

    pub fn r(parent: &RNode<H, T>) -> Self {
        Self::R(Rc::downgrade(parent))
    }

    pub fn of(parent: &RNode<H, T>, dir: usize) -> Self {
      match dir {
        0 => Self::l(parent),
        1 => Self::r(parent),
        _ => unreachable!(),
      }
    }

    pub fn node(&self) -> RNode<H, T> {
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

    pub fn set_child(&self, child: Option<RNode<H, T>>) {
        let (parent, dir) = match self {
            L(parent) => (parent, 0),
            R(parent) => (parent, 1),
        };
        let parent = Weak::upgrade(parent).unwrap();
        parent.borrow_mut().children[dir] = child;
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
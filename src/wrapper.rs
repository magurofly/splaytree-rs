use std::{cell::RefCell, cmp::Ordering::*, rc::Rc};
use super::{helper::*, node::*, parent::*};

pub trait Wrapper<T, H: Helper<T> = DefaultHelper>: Sized {
  fn node(&self) -> &Node<T, H>;
  fn node_mut(&self) -> &mut Node<T, H>;

  fn new(value: T) -> RNode<T, H> {
    Rc::new(RefCell::new(Node::new(value)))
  }

  /// 親が存在しない場合 `true`
  fn is_root(&self) -> bool {
    self.node().parent().is_none()
  }

  /// 親を返す
  fn parent(&self) -> Option<Parent<T, H>> {
    self.node().parent().cloned()
  }

  /// 親を設定する
  fn set_parent(&self, parent: Option<Parent<T, H>>) {
    // もとの親を切り離す
    self.take_parent();
    // 新しい親をつなぐ（親→子）
    if let Some(parent_new) = &parent {
      let dir = parent_new.dir();
      *parent_new.node_mut().child_mut(dir) = Some(self.to_ref().clone());
    }
    // 新しい親をつなぐ（子→親）
    *self.node_mut().parent_mut() = parent;
  }

  /// 親を取り去る
  fn take_parent(&self) -> Option<Parent<T, H>> {
    let parent = self.node_mut().parent_mut().take();
    if let Some(parent) = &parent {
      let dir = parent.dir();
      parent.node_mut().child_mut(dir).take();
    }
    parent
  }

  /// 子を返す
  fn child(&self, dir: usize) -> Option<RNode<T, H>> {
    self.node().child(dir).cloned()
  }

  /// 子を設定する
  fn set_child(&self, dir: usize, child: Option<RNode<T, H>>) {
    // もとの子を切り離す
    self.take_child(dir);
    if let Some(child) = &child {
      // 子のもとの親を切り離す（親→子）
      if let Some(parent_old) = child.node_mut().parent_mut().take() {
        let dir = parent_old.dir();
        parent_old.node_mut().child_mut(dir).take();
      }
      // 新しい子をつなぐ（子→親）
      *child.node_mut().parent_mut() = Some(Parent::of(self.to_ref(), dir));
    }
    // 新しい子をつなぐ（親→子）
    *self.node_mut().child_mut(dir) = child;
  }

  /// 子を取り去る
  fn take_child(&self, dir: usize) -> Option<RNode<T, H>> {
    let child = self.node_mut().child_mut(dir).take();
    if let Some(child) = &child {
      child.set_parent(None);
    }
    child
  }

  /// 値を取り出す
  fn tap<U, F: FnMut(&mut T) -> U>(&self, mut f: F) -> U {
    (f)(self.node_mut().value_mut())
  }

  /// 部分木のノード数
  fn size(&self) -> usize {
    self.node().size()
  }

  /// 部分木の深さ
  fn depth(&self) -> usize {
    self.node().depth()
  }

  /// `dir` 方向に回転する
  /// 新しく親になったノードを返す
  fn rotate(&self, dir: usize) -> RNode<T, H> {
    assert!(dir < 2);
    let child = self.node_mut().child_mut(1 ^ dir).take().unwrap();
    child.set_parent(self.take_parent());
    self.set_child(1 ^ dir, child.take_child(dir));
    child.set_child(dir, Some(self.to_ref().clone()));
    child
  }

  // internal

  fn to_ref(&self) -> &RNode<T, H> {
    unsafe { std::mem::transmute::<_, &RNode<T, H>>(self) }
  }
  
  fn zig(&self, dir: usize) {
    self.rotate(dir);
  }
  
  fn zig_zig(&self, dir: usize) {
    self.rotate(dir).rotate(dir);
  }
  
  fn zig_zag(&self, dir: usize) {
    self.node().child(1 ^ dir).unwrap().rotate(1 ^ dir);
    self.rotate(dir);
  }

  fn eval(&self) {
    let mut depth = 1;
    let mut size = 1;
    for dir in 0 .. 2 {
      if let Some(child) = self.node().child(dir) {
        depth = depth.max(child.depth() + 1);
        size += child.size();
      }
    }
    *self.node_mut().depth_mut() = depth;
    *self.node_mut().size_mut() = size;
    H::eval(self.to_ref());
  }

  fn splay(&self) {
    while let Some(parent) = self.parent() {
      if let Some(grandparent) = parent.parent() {
        if parent.dir() == grandparent.dir() {
          grandparent.zig_zig(1 ^ grandparent.dir());
        } else {
          grandparent.zig_zag(1 ^ grandparent.dir());
        }
      } else {
        self.zig(1 ^ parent.dir());
      }
    }
  }

  fn find<F: FnMut(&T) -> std::cmp::Ordering>(&self, mut f: F) -> Option<RNode<T, H>> {
    let mut optnode = Some(self.to_ref().clone());
    while let Some(node) = optnode {
      match (f)(node.node().value()) {
        Equal => {
          node.splay();
          return Some(node.to_ref().clone());
        }
        Greater => {
          optnode = self.child(0);
        }
        Less => {
          optnode = self.child(1);
        }
      }
    }
    None
  }
}
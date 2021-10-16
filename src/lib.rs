pub mod helper;
pub mod node;
pub mod parent;
pub mod wrapper;

/**
 * ```
 * use splaytree::*;
 * let node = new_splaytree_node::<i32, SplayTreeDefaultHelper>(1);
 * ```
 */

pub use helper::Helper as SplayTreeHelper;
pub use helper::DefaultHelper as SplayTreeDefaultHelper;
pub use wrapper::Wrapper as SplayTreeNode;

pub fn new_splaytree_node<T, H: SplayTreeHelper<T>>(value: T) -> node::RNode<T, H> {
  std::rc::Rc::new(std::cell::RefCell::new(node::Node::new(value)))
}

#[cfg(test)]
mod tests {
  use crate::helper::DefaultHelper;

use super::*;

  #[test]
  fn make_tree() {
    let new = |value| new_splaytree_node::<_, DefaultHelper>(value);
    let x = new(1);
    let y = new(2);

    eprintln!("{:?}", x);
    eprintln!("{:?}", y);
  }
}

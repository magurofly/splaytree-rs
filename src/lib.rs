pub mod helper;
pub mod node;
pub mod wrapper;

pub use helper::Helper as SplayTreeHelper;
pub use node::Node as SplayTreeNode;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

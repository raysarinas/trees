use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]

enum NodeColor {
    Red,
    Black,
}

type Tree = Rc<RefCell<TreeNode<u32>>>;
type RedBlackTree= Option<Tree>;

struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: RedBlackTree,
    left: RedBlackTree,
    right: RedBlackTree,
}

struct RBTree<T> {
    root: TreeNode<T>,
    len: usize
}

impl<T> RBTree<T> {
    // fn rotate() {

    // }
    
    // fn fixDelColor() {
    
    // }

    // fn fixInsColor() {
    
    // }
    
    pub fn delete_node(&mut self) {
        self.len -= 1;
    }
    
    pub fn height() {
    
    }

    pub fn insert_node(&mut self) {
        self.len += 1;
    }
    
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn print() {
    
    }

    pub fn size(&self) -> usize {
        self.len
    }
}
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

impl<T> TreeNode<T> {
    // fn rotate() {

    // }
    
    // fn fixDelColor() {
    
    // }

    // fn fixInsColor() {
    
    // }
    
    pub fn delete_node() {
    
    }
    
    pub fn height() {
    
    }

    pub fn insert_node() {
    
    }
    
    pub fn is_empty() {
        
    }

    pub fn print() {
    
    }

    pub fn size() {
    
    }
}
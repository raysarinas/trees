use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq)]

enum NodeColor {
    Red,
    Black,
}

type Tree<T> = Rc<RefCell<TreeNode<T>>>;
type RedBlackTree<T> = Option<Tree<T>>;

#[derive(Debug)]
struct TreeNode<T>  where T: Clone + PartialEq + Ord {
    pub color: NodeColor,
    pub value: Option<T>,
    pub parent: RedBlackTree<T>,
    left: RedBlackTree<T>,
    right: RedBlackTree<T>,
}

impl<T> TreeNode<T>  where T: Clone + PartialEq + Ord {
    fn new(val: T) -> TreeNode<T> {
        TreeNode {
            color: NodeColor::Black,
            value: Some(val),
            parent: None,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct RBTree<T> where T: Clone + PartialEq + Ord {
    root: TreeNode<T>,
    len: usize
}

impl<T> RBTree<T> where T: Clone + PartialEq + Ord {
    pub fn new(val: T) -> RBTree<T> {
        RBTree {
            root: TreeNode::new(val),
            len: 0,
        }
    }
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
    
    fn set_left(&self, node: TreeNode<T>, value: &T) -> TreeNode<T> {

        match node.value {
            Some(curr_node) => {
                match &node.left {
                    Some(left) => TreeNode::new(value.clone()),
                    None => TreeNode::new(value.clone()),
                }
            },
            None => panic!("bad"),
        }
    }

    pub fn insert_node(&mut self, value: T) {

        if self.is_empty() {
            self.root = TreeNode::new(value)
        }
        let mut root = &mut self.root;
        let mut curr_tree = Some(root);

        // 1. Starting from the root node or with a current node
        while let Some(curr_node) = curr_tree {

            let curr_node_value = curr_node.value.as_ref();
            if curr_node_value.is_some() {
                match curr_node_value.unwrap().cmp(&value) {
                    // this is broken
                    Ordering::Less => curr_tree = Some(&mut self.set_left(*curr_node, &value)), //self.set_right(&curr_node, value), //curr_node.right.clone().unwrap(),
                    Ordering::Equal => return,
                    Ordering::Greater => curr_tree = Some(&mut self.set_left(*curr_node, &value)),
                }
            }
        }

        *curr_tree.unwrap() = TreeNode::new(value);
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
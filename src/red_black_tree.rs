use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]

enum NodeColor {
    Red,
    Black,
}

type Tree<T> = Rc<RefCell<TreeNode<T>>>;
type RedBlackTree<T> = Option<Tree<T>>;

pub struct TreeNode<T> {
    color: NodeColor,
    value: T,
    parent: RedBlackTree<T>,
    left: RedBlackTree<T>,
    right: RedBlackTree<T>
}

pub trait TreeNodeTraits<T> {
    fn color(&self) -> NodeColor;
    fn value(&self) -> T;
    fn parent(&self) -> RedBlackTree<T>;
    fn left(&self) -> RedBlackTree<T>;
    fn right(&self) -> RedBlackTree<T>;
    fn set_color(&self, color: NodeColor);
    fn set_value(&self, value: T);
    fn set_parent(&self, parent: RedBlackTree<T>);
    fn set_left(&self, left: RedBlackTree<T>);
    fn set_right(&self, right: RedBlackTree<T>);
}

impl<T> TreeNode<T> {
    pub fn new(val: T) -> TreeNode<T> {
        TreeNode {
            color: NodeColor::Black,
            value: val,
            parent: None,
            left: None,
            right: None
        }
    }

    // fn is_some(& self) -> bool {
    //     match self.value {
    //         Some(_) => true,
    //         None => false,
    //     }
    // }

    fn new_wrapped(val: T) -> RedBlackTree<T> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    pub fn va(&self) -> T {
        self.value()
    }

    // fn unwrap(node: RedBlackTree<T>) -> TreeNode<T> {
    //     node.as_ref().unwrap().as_ref().get_mut()
    // }
}

impl<T> TreeNodeTraits<T> for TreeNode<T> {
    fn color(&self) -> NodeColor {
        self.color.clone()
    }

    fn value(&self) -> T {
        self.value
    }

    fn parent(&self) -> RedBlackTree<T> {
        self.parent.clone()
    }

    fn left(&self) -> RedBlackTree<T> {
        self.left.clone()
    }

    fn right(&self) -> RedBlackTree<T> {
        self.right.clone()
    }

    fn set_color(&self, color: NodeColor) {
        self.color = color;
    }

    fn set_value(&self, value: T) {
        self.value = value;
    }

    fn set_parent(&self, parent: RedBlackTree<T>) {
        self.parent = parent;
    }

    fn set_left(&self, left: RedBlackTree<T>) {
        self.left = left;
    }

    fn set_right(&self, right: RedBlackTree<T>) {
        self.right = right;
    }
}

pub struct RBTree<T> {
    root: RedBlackTree<T>,
    len: usize
}

impl<T> RBTree <T> where T: Ord + PartialEq + PartialOrd {
    pub fn new() -> RBTree<T> {
        RBTree {
            root: None,
            len: 0
        }
    }
    // fn rotate() {

    // }
    
    // fn fixDelColor() {
    
    // }

    fn fixInsColor() {
    
    }
    
    pub fn delete_node(&mut self) {
        self.len -= 1;
    }
    
    pub fn height(&self) -> usize {
        // TODO: add match statements for left and right heights
        // need to borrow and match stuff. see commented code below

        // let left_height = self.root.left;
        // let right_height = self.root.right;

        // let left_height = match *self.left.borrow() {
        //     Some(ref left) => left.height(),
        //     None => 0,
        // };
        // let right_height = match *self.right.borrow() {
        //     Some(ref right) => right.height(),
        //     None => 0,
        // };
        // if left_height > right_height {
        //     left_height += 1;
        // } else {
        //     right_height += 1;
        // }
        0
    }

    pub fn insert_node2(&mut self, value: T) {
        // if self.len == 0 {
        //     self.root = TreeNode::new_wrapped(value);
        // } else {
        //     let root_unwrapped = self.root.unwrap().borrow();
        //     let mut new_node = TreeNode::new(value);
        //     let mut subroot = root_unwrapped;
        //     let is_left_child = true;

        //     while new_node.value != root_unwrapped.value && new_node.parent.is_none() {

        //         if !subroot.is_some() {

        //             // insert at empty node
        //             new_node.parent = subroot.parent;
        //             let mut unwrapped_parent = subroot.parent.unwrap().borrow();

        //             if is_left_child {
        //                 unwrapped_parent.left = TreeNode::new_wrapped(value);
        //             } else {
        //                 unwrapped_parent.right = TreeNode::new_wrapped(value);
        //             }

        //         } else if value < subroot.value.unwrap() {
        //             subroot = subroot.left.unwrap().borrow();
        //             is_left_child = true;
                    
        //         } else { //value > curr_node_unwrapped.value {
        //             subroot = subroot.right.unwrap().borrow();
        //             is_left_child = false;
        //         }
        //     }

        //     // rebalance the tree here i guess and fix colors
        //     fixInsColor();
        // }
    }


    // pub fn insert_node(&mut self, value: T) {
    //     if self.len == 0 {
    //         self.root = TreeNode::new_wrapped(value);
    //     } else {
    //         let curr_node = &self.root;
    //         let curr_node_unwrapped = curr_node.as_ref().unwrap().as_ref().get_mut();

    //         if value == curr_node_unwrapped.value {
    //             return
    //         }



    //         // loop  {}
    //         let curr_node = if value < curr_node_unwrapped.value {
    //             curr_node_unwrapped.left
    //         } else {
    //             curr_node_unwrapped.right
    //         };

    //         let new_node = TreeNode::new(value);
    //         new_node.parent = curr_node_unwrapped.parent;

    //         match curr_node {
    //             // _ => Some(Rc::new(RefCell::new(new_node))),
    //             Some(sub_node) => self.insert_node(value),
    //             None => {
    //                 // let temp = Some(Rc::new(RefCell::new(TreeNode::new(value))));
    //                 // *curr_node = temp;
    //                 *curr_node = Some(Rc::new(RefCell::new(new_node)));
    //             },
    //             // &mut Some(ref mut sub_node) => sub_node.insert_node(value),
    //             // &mut None => {
    //             //     let temp = TreeNode { value: value, left_child: None, right_child: None};
    //             //     let boxed_node = Some(Box::new(temp));
    //             //     *new_node = boxed_node;
    //             // }
    //         }

    //         // balance the tree here i guess. need to pass in node tho
    //         fixInsColor();
    //     }
    //     self.len += 1;
    // }
    
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn print() {
    
    }

    pub fn size(&self) -> usize {
        self.len
    }
}
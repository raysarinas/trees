use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]

/********** TreeNode Helpers **********/

pub enum NodeColor {
    Red,
    Black,
}

pub type TreeNode<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Node<T> {
    color: NodeColor,
    value: Option<T>,
    parent: TreeNode<T>,
    left: TreeNode<T>,
    right: TreeNode<T>
}

pub trait NodeTraits<T> {
    fn new(val: T) -> TreeNode<T>;
    fn unwrapped(&self) -> Rc<RefCell<Node<T>>>;
    fn color(&self) -> NodeColor;
    fn value(&self) -> Option<T>;
    fn parent(&self) -> TreeNode<T>;
    fn left(&self) -> TreeNode<T>;
    fn right(&self) -> TreeNode<T>;
    fn set_color(&self, color: NodeColor);
    fn set_value(&self, value: T);
    fn set_parent(&self, parent: TreeNode<T>);
    fn set_left(&self, left: TreeNode<T>);
    fn set_right(&self, right: TreeNode<T>);
}

impl<T> NodeTraits<T> for TreeNode<T> where T: Copy {
    fn new(val: T) -> TreeNode<T> {
        Some(Rc::new(RefCell::new(Node {
            color: NodeColor::Black,
            value: Some(val),
            parent: None,
            left: None,
            right: None
        })))
    }

    fn unwrapped(&self) -> Rc<RefCell<Node<T>>> {
        match self {
            Some(tree_node) => Rc::clone(&tree_node),
            None => panic!("Error unwrapping tree node")
        }
    }

    fn color(&self) -> NodeColor {
        match self {
            Some(tree_node) => tree_node.borrow().color.clone(),
            None => panic!("Error getting tree node color")
        }
    }

    fn value(&self) -> Option<T> {
        match self {
            Some(tree_node) => tree_node.borrow().value,
            None => panic!("Error getting tree node value")
        }
    }

    fn parent(&self) -> TreeNode<T> {
        match self {
            Some(tree_node) => tree_node.borrow().parent.clone(),
            None => panic!("Error getting tree node parent")
        }
    }

    fn left(&self) -> TreeNode<T> {
        match self {
            Some(tree_node) => tree_node.borrow().left.clone(),
            None => panic!("Error getting tree node left child")
        }
    }

    fn right(&self) -> TreeNode<T> {
        match self {
            Some(tree_node) => tree_node.borrow().right.clone(),
            None => panic!("Error getting tree node right child")
        }
    }

    fn set_color(&self, color: NodeColor) {
        self.unwrapped().borrow_mut().color = color;
    }

    fn set_value(&self, value: T) {
        self.unwrapped().borrow_mut().value = Some(value);
    }

    fn set_parent(&self, parent: TreeNode<T>) {
        self.unwrapped().borrow_mut().parent = parent;
    }

    fn set_left(&self, left: TreeNode<T>) {
        self.unwrapped().borrow_mut().left = left;
    }

    fn set_right(&self, right: TreeNode<T>) {
        self.unwrapped().borrow_mut().right = right;
    }
}

/********** RBTree Helpers **********/
pub struct RBTree<T> {
    root: TreeNode<T>,
    len: usize
}

pub trait RBTreeTraits<T> {
    fn new() -> RBTree<T>;
    fn height(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
    fn rotate_left(&self, node: TreeNode<T>);
    fn rotate_right(&self, node: TreeNode<T>);
    fn fix_ins_color(&self, node: TreeNode<T>);
    fn fix_del_color(&self, node: TreeNode<T>);
    fn insert_node(&self, value: T);
    fn delete_node(&self, value: T);
    fn print(&self);
}

impl<T> RBTreeTraits<T> for RBTree<T> {
    fn new() -> RBTree<T> {
        RBTree {
            root: None,
            len: 0
        }
    }

    // TODO
    fn height(&self) -> usize {
        // // TODO: add match statements for left and right heights
        // // need to borrow and match stuff. see commented code below

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

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn size(&self) -> usize {
        self.len
    }

    // TODO
    fn rotate_left(&self, node: TreeNode<T>) {

    }

    // TODO
    fn rotate_right(&self, node: TreeNode<T>) {

    }

    // TODO
    fn fix_ins_color(&self, node: TreeNode<T>) {

    }

    // TODO
    fn insert_node(&self, value: T) {

    }

    // TODO
    fn fix_del_color(&self, node: TreeNode<T>) {

    }

    // TODO
    fn delete_node(&self, value: T) {

    }

    // TODO
    fn print(&self) {

    }
}

    // pub fn insert_node2(&mut self, value: T) {
    //     // if self.len == 0 {
    //     //     self.root = Node::new_wrapped(value);
    //     // } else {
    //     //     let root_unwrapped = self.root.unwrap().borrow();
    //     //     let mut new_node = Node::new(value);
    //     //     let mut subroot = root_unwrapped;
    //     //     let is_left_child = true;

    //     //     while new_node.value != root_unwrapped.value && new_node.parent.is_none() {

    //     //         if !subroot.is_some() {

    //     //             // insert at empty node
    //     //             new_node.parent = subroot.parent;
    //     //             let mut unwrapped_parent = subroot.parent.unwrap().borrow();

    //     //             if is_left_child {
    //     //                 unwrapped_parent.left = Node::new_wrapped(value);
    //     //             } else {
    //     //                 unwrapped_parent.right = Node::new_wrapped(value);
    //     //             }

    //     //         } else if value < subroot.value.unwrap() {
    //     //             subroot = subroot.left.unwrap().borrow();
    //     //             is_left_child = true;
                    
    //     //         } else { //value > curr_node_unwrapped.value {
    //     //             subroot = subroot.right.unwrap().borrow();
    //     //             is_left_child = false;
    //     //         }
    //     //     }

    //     //     // rebalance the tree here i guess and fix colors
    //     //     fixInsColor();
    //     // }
    // }


    // // pub fn insert_node(&mut self, value: T) {
    // //     if self.len == 0 {
    // //         self.root = Node::new_wrapped(value);
    // //     } else {
    // //         let curr_node = &self.root;
    // //         let curr_node_unwrapped = curr_node.as_ref().unwrap().as_ref().get_mut();

    // //         if value == curr_node_unwrapped.value {
    // //             return
    // //         }



    // //         // loop  {}
    // //         let curr_node = if value < curr_node_unwrapped.value {
    // //             curr_node_unwrapped.left
    // //         } else {
    // //             curr_node_unwrapped.right
    // //         };

    // //         let new_node = Node::new(value);
    // //         new_node.parent = curr_node_unwrapped.parent;

    // //         match curr_node {
    // //             // _ => Some(Rc::new(RefCell::new(new_node))),
    // //             Some(sub_node) => self.insert_node(value),
    // //             None => {
    // //                 // let temp = Some(Rc::new(RefCell::new(Node::new(value))));
    // //                 // *curr_node = temp;
    // //                 *curr_node = Some(Rc::new(RefCell::new(new_node)));
    // //             },
    // //             // &mut Some(ref mut sub_node) => sub_node.insert_node(value),
    // //             // &mut None => {
    // //             //     let temp = Node { value: value, left_child: None, right_child: None};
    // //             //     let boxed_node = Some(Box::new(temp));
    // //             //     *new_node = boxed_node;
    // //             // }
    // //         }

    // //         // balance the tree here i guess. need to pass in node tho
    // //         fixInsColor();
    // //     }
    // //     self.len += 1;
    // // }
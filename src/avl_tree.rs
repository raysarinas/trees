use std::cell::RefCell;
use std::rc::Rc;
use std::mem::{replace, swap};
use std::cmp::max;

// #[derive(Clone, Debug, PartialEq)]

pub type AVLTreeNode<T> = Option<Rc<RefCell<AVLNode<T>>>>;

pub struct AVLNode<T> {
    value: Option<T>,
    parent: AVLTreeNode<T>,
    left: AVLTreeNode<T>,
    right: AVLTreeNode<T>,
    height: isize,
}

pub trait NodeTraits<T> {
    fn new(val: T) -> AVLTreeNode<T>;
    fn unwrapped(&self) -> Rc<RefCell<AVLNode<T>>>;
    fn compare(&self, node: &AVLTreeNode<T>) -> bool;
    // fn node_height(&self) -> usize;
    fn print_traversal(&self);
    // fn color(&self) -> NodeColor;
    fn value(&self) -> Option<T>;
    fn parent(&self) -> AVLTreeNode<T>;
    fn left(&self) -> AVLTreeNode<T>; 
    fn right(&self) -> AVLTreeNode<T>;
    fn height(&self) -> isize; // AVL
    fn grandparent(&self) -> AVLTreeNode<T>;
    fn uncle(&self) -> AVLTreeNode<T>;
    fn recalc_height(&mut self); // AVL
    fn set_value(&self, value: T);
    fn set_parent(&mut self, parent: AVLTreeNode<T>);
    fn set_left(&mut self, left: AVLTreeNode<T>);
    fn set_right(&mut self, right: AVLTreeNode<T>);
    fn set_height(&self, value: isize); // AVL
    fn find_node(&self, value: T) -> AVLTreeNode<T>;
}

impl<T> NodeTraits<T> for AVLTreeNode<T> where T: Copy + PartialOrd + std::fmt::Debug {
    fn new(val: T) -> AVLTreeNode<T> {
        let tree_node = Some(Rc::new(RefCell::new(AVLNode {
            value: Some(val),
            parent: None,
            left: None,
            right: None,
            height: 1,
        })));
        tree_node.left().set_parent(tree_node.clone());
        tree_node.right().set_parent(tree_node.clone());
        tree_node
    }

    fn unwrapped(&self) -> Rc<RefCell<AVLNode<T>>> {
        match self {
            Some(tree_node) => Rc::clone(&tree_node),
            None => panic!("Error unwrapping tree node")
        }
    }

    fn compare(&self, node: &AVLTreeNode<T>) -> bool {
        if self.is_none() || node.is_none() {
            return false
        }
        Rc::ptr_eq(&self.unwrapped(), &node.unwrapped())
    }

    // node_height

    fn print_traversal(&self) {
        if self.is_some() && self.value().is_some() {
            self.left().print_traversal();
            print!("{:?} ", self.value().unwrap());
            self.right().print_traversal();
        }
    }

    fn value(&self) -> Option<T> {
        match self {
            Some(tree_node) => tree_node.borrow().value,
            None => None
        }
    }

    fn parent(&self) -> AVLTreeNode<T> {
        match self {
            Some(tree_node) => tree_node.borrow().parent.clone(),
            None => None
        }
    }

    fn left(&self) -> AVLTreeNode<T> {
        match self {
            Some(tree_node) => tree_node.borrow().left.clone(),
            None => None
        }
    }

    fn right(&self) -> AVLTreeNode<T> {
        match self {
            Some(tree_node) => tree_node.borrow().right.clone(),
            None => None
        }
    }

    fn height(&self) -> isize {
        match self {
            Some(tree_node) => tree_node.borrow().height,
            None => 0
        }
    }

    fn grandparent(&self) -> AVLTreeNode<T> {
        self.parent().parent()
    }

    fn uncle(&self) -> AVLTreeNode<T> {
        if self.grandparent().left().is_none() || self.grandparent().right().is_none() {
            None
        } else if self.parent().compare(&self.grandparent().left()) {
            self.grandparent().right()
        } else {
            self.grandparent().left()
        }
    }

    fn set_value(&self, value: T) {
        self.unwrapped().borrow_mut().value = Some(value);
    }

    fn set_parent(&mut self, parent: AVLTreeNode<T>) {
        match self {
            Some(tree_node) => tree_node.borrow_mut().parent = parent,
            None => *self = Some(Rc::new(RefCell::new(AVLNode {
                value: self.value(),
                parent: parent,
                left: self.left(),
                right: self.right(),
                height: self.height(),
            })))
        }
    }

    fn set_left(&mut self, left: AVLTreeNode<T>) {
        match self {
            Some(tree_node) => tree_node.borrow_mut().left = left,
            None => *self = Some(Rc::new(RefCell::new(AVLNode {
                value: self.value(),
                parent: self.parent(),
                left: left,
                right: self.right(),
                height: self.height(),
            })))
        }
    }

    fn set_right(&mut self, right: AVLTreeNode<T>) {
        match self {
            Some(tree_node) => tree_node.borrow_mut().right = right,
            None => *self = Some(Rc::new(RefCell::new(AVLNode {
                value: self.value(),
                parent: self.parent(),
                left: self.left(),
                right: right,
                height: self.height(),
            })))
        }
    }

    fn set_height(&self, value: isize) {
        self.unwrapped().borrow_mut().height = value;
    }

    fn find_node(&self, value: T) -> AVLTreeNode<T> {
        match self {
            Some(_) => {
                if value == self.value().unwrap() {
                    self.clone()
                } else if value < self.value().unwrap() {
                    self.left().find_node(value)
                }else {
                    self.right().find_node(value)
                    }
            }, 
            None => None
        }
    }

    fn recalc_height(&mut self) {
        let left = self.left();
        let right = self.right();
        self.set_height(1 + max(left.height(), right.height()));
    }
}

// fn rotate_left(&mut self) {
//     if self.right.is_none() {
//         return
//     }

//     let right_child = self.right.as_mut().unwrap().borrow();
//     let right_left = right_child.left.take();
//     let right_right = right_child.right.take();

//     // Replace current right child with right grandchild
//     let mut new_left_tree = replace(&mut self.right, right_right);
//     swap(&mut self.value, &mut new_left_tree.as_mut().unwrap().borrow().value);

//     let left_tree = self.left.take();
//     let new_left_node = new_left_tree.as_mut().unwrap().borrow();
    
//     new_left_node.right = right_left;
//     new_left_node.left = left_tree;
//     self.left = new_left_tree;

//     if let Some(node) = self.left.as_mut() {
//         self.left.as_mut().unwrap().borrow().recalc_height();
//     }

//     self.recalc_height();
// }
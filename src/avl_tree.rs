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
    // helper functions
    fn new(val: T) -> AVLTreeNode<T>;
    fn unwrapped(&self) -> Rc<RefCell<AVLNode<T>>>;
    fn compare(&self, node: &AVLTreeNode<T>) -> bool;
    fn find_node(&self, value: T) -> AVLTreeNode<T>;
    fn print_traversal(&self);
    fn count_leaves(&self) -> usize;
    fn recalc_height(&mut self); // AVL
    
    // getters for node properties and family members
    fn height(&self) -> isize; // AVL
    fn value(&self) -> Option<T>;
    fn parent(&self) -> AVLTreeNode<T>;
    fn left(&self) -> AVLTreeNode<T>; 
    fn right(&self) -> AVLTreeNode<T>;
    fn grandparent(&self) -> AVLTreeNode<T>;
    fn uncle(&self) -> AVLTreeNode<T>;
    fn sibling(&self) -> AVLTreeNode<T>;

    // setters for node properties
    fn set_height(&self, value: isize); // AVL
    fn set_value(&self, value: T);
    fn set_parent(&mut self, parent: AVLTreeNode<T>);
    fn set_left(&mut self, left: AVLTreeNode<T>);
    fn set_right(&mut self, right: AVLTreeNode<T>);
    
    
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

    fn print_traversal(&self) {
        if self.is_some() && self.value().is_some() {
            self.left().print_traversal();
            print!("{:?} ", self.value().unwrap());
            self.right().print_traversal();
        }
    }

    fn count_leaves(&self) -> usize {
        if self.is_none() {
            0
        } else if self.left().is_none() && self.right().is_none() {
            1
        } else {
            self.left().count_leaves() + self.right().count_leaves()
        }
    }

    fn recalc_height(&mut self) {
        let left = self.left();
        let right = self.right();
        self.set_height(1 + max(left.height(), right.height()));
    }

    fn height(&self) -> isize {
        match self {
            Some(tree_node) => tree_node.borrow().height,
            None => 0
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

    fn sibling(&self) -> AVLTreeNode<T> {
        match self.compare(&self.parent().left()) {
            true => self.parent().right(),
            false => self.parent().left(),
        }
    }

    fn set_height(&self, value: isize) {
        self.unwrapped().borrow_mut().height = value;
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

}

/******************** AVL Tree Helpers ********************/

pub struct AVLTree<T> {
    root: AVLTreeNode<T>,
}

pub trait AVLTreeTraits<T> {
    fn new() -> AVLTree<T>;
    fn height(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
    fn search(&self, value: T) -> AVLTreeNode<T>;
    fn rotate(&mut self, node: &AVLTreeNode<T>, direction: bool);
    fn fix_insert_height(&mut self, node: &AVLTreeNode<T>);
    fn fix_delete_height(&mut self, node: &AVLTreeNode<T>);
    fn insert_node(&mut self, value: T);
    fn delete_node(&mut self, value: T);
    fn print(&self);
    fn count_leaves(&self) -> usize;
}

impl<T> AVLTreeTraits<T> for AVLTree<T> where T: Copy + PartialOrd + std::fmt::Debug {
    fn new() -> AVLTree<T> {
        AVLTree {
            root: None,
        }
    }

    fn height(&self) -> usize {
        0
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    // DO WE NEED SIZE?
    fn size(&self) -> usize {
        1
    }

    fn search(&self, value: T) -> AVLTreeNode<T> {
        self.root.find_node(value)
    }

    // TODO
    fn rotate(&mut self, node: &AVLTreeNode<T>, direction: bool) {

    }

    // TODO
    fn fix_insert_height(&mut self, node: &AVLTreeNode<T>) {
        println!("inside fixing insert height");
    }

    // TODO
    fn fix_delete_height(&mut self, node: &AVLTreeNode<T>) {

    }

    // TODO
    fn insert_node(&mut self, value: T) {

        // REGULAR BINARY SEARCH TREE INSERTION 
        let mut new_node = AVLTreeNode::new(value);

        if self.is_empty() {
            self.root = new_node.clone();
        }
        else if self.search(value).is_some() {
            println!("Value already exists!");
            return;
        }

        else {
            let mut curr_node = self.root.clone();
            let mut curr_node_parent: AVLTreeNode<T> = None;
            let mut is_left_child = true;

            // find empty node
            while curr_node.value().is_some() {
                curr_node_parent = curr_node.clone();
                if value < curr_node.value().unwrap() {
                    curr_node = curr_node.left();
                    is_left_child = true;
                } else {
                    curr_node = curr_node.right();
                    is_left_child = false;
                }
            }
            
            // place new_node in found spot
            if curr_node_parent.is_some() {
                new_node.set_parent(curr_node_parent);
                if is_left_child {
                    new_node.parent().set_left(new_node.clone());
                } else {
                    new_node.parent().set_right(new_node.clone());
                }
            } else {
                panic!("Current node has no parent!");
            }
        }

        // TODO: AVL REBALANCING HERE
        self.fix_insert_height(&new_node);
    }

    // TODO
    fn delete_node(&mut self, value: T) {

    }

    fn print(&self) {
        if self.is_empty() {
            println!("Tree is empty. Nothing to print.");
        } else {
            println!("Root: {:?}", self.root.value().unwrap());
            self.root.print_traversal();
            println!();
        }
    }

    fn count_leaves(&self) -> usize {
        self.root.count_leaves()
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
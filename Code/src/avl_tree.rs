use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::max;
use crate::tree::{TreeBase, NodeTraits};

static ROTATE_LEFT: bool = true;
static ROTATE_RIGHT: bool = false;

pub type AVLTreeNode<T> = Option<Rc<RefCell<AVLNode<T>>>>;

#[derive(Clone, Debug, PartialEq)]
pub struct AVLNode<T> {
    value: Option<T>,
    parent: AVLTreeNode<T>,
    left: AVLTreeNode<T>,
    right: AVLTreeNode<T>,
    height: isize,
}

pub trait AVLNodeTraits<T> {
    // helper functions
    fn new(val: T) -> AVLTreeNode<T>;
    fn unwrapped(&self) -> Rc<RefCell<AVLNode<T>>>;
    fn compare(&self, node: &AVLTreeNode<T>) -> bool;
    fn find_node(&self, value: T) -> AVLTreeNode<T>;

    // AVL specific helper functions
    fn recalc_height(&mut self);
    fn get_balance(&self) -> isize;
    
    // getters for node properties and family members
    fn parent(&self) -> AVLTreeNode<T>;
    fn left(&self) -> AVLTreeNode<T>; 
    fn right(&self) -> AVLTreeNode<T>;
    fn grandparent(&self) -> AVLTreeNode<T>;
    fn uncle(&self) -> AVLTreeNode<T>;
    fn sibling(&self) -> AVLTreeNode<T>;
    fn height(&self) -> isize; // AVL specific node property

    // setters for node properties
    fn set_parent(&mut self, parent: AVLTreeNode<T>);
    fn set_left(&mut self, left: AVLTreeNode<T>);
    fn set_right(&mut self, right: AVLTreeNode<T>);
    fn set_height(&self, value: isize); // AVL specific node property
}


impl<T> NodeTraits<T> for AVLTreeNode<T> where T: Copy + PartialOrd + std::fmt::Debug {

    // print in-order traversal of tree
    fn print_traversal(&self) {
        if self.is_some() && self.value().is_some() {
            self.left().print_traversal();
            print!("{:?} ", self.value().unwrap());
            self.right().print_traversal();
        }
    }

    // count number of leaves in a tree
    fn count_leaves(&self) -> usize {
        if self.value().is_none() {
            0
        } else if self.left().value().is_none() && self.right().value().is_none() {
            1
        } else {
            self.left().count_leaves() + self.right().count_leaves()
        }
    }

    // retrieve a vector containing each node in a tree with respective depth
    fn get_depth_vec(&self) -> Vec<(T, usize)> {
        let mut vec: Vec<(T, usize)> = Vec::new();
        self.calc_depth(0, &mut vec);
        vec.sort_by(|a, b| b.1.cmp(&a.1)); // sort by depth
        vec.dedup_by(|a, b| a.0 == b.0); // remove duplicate nodes
        vec
    }

    // calculate the depth of each node and store them in a given vector
    fn calc_depth(&self, depth: usize, vec: &mut Vec<(T, usize)>) {
        match self.value() {
            Some(_) => {
                vec.push((self.value().unwrap(), depth));
                self.left().calc_depth(depth+1, vec);
                self.right().calc_depth(depth+1, vec)
            }
            None => {},
        }
    }

    // required getters for node properties
    fn value(&self) -> Option<T> {
        match self {
            Some(tree_node) => tree_node.borrow().value,
            None => None
        }
    }

    // setters for node properties
    fn set_value(&self, value: T) {
        self.unwrapped().borrow_mut().value = Some(value);
    }
}

impl<T> AVLNodeTraits<T> for AVLTreeNode<T> where T: Copy + PartialOrd + std::fmt::Debug {

    // return new instance of an AVL tree node
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

    // return unwrapped tree node (reference counter)
    fn unwrapped(&self) -> Rc<RefCell<AVLNode<T>>> {
        match self {
            Some(tree_node) => Rc::clone(&tree_node),
            None => panic!("Error unwrapping tree node")
        }
    }

    // compare if two nodes are the same or not
    fn compare(&self, node: &AVLTreeNode<T>) -> bool {
        if self.is_none() || node.is_none() {
            return false
        }
        Rc::ptr_eq(&self.unwrapped(), &node.unwrapped())
    }

    // search for a node with given value
    fn find_node(&self, value: T) -> AVLTreeNode<T> {
        match self.value() {
            Some(_) => {
                if value == self.value().unwrap() {
                    self.clone()
                } else if value < self.value().unwrap() {
                    self.left().find_node(value)
                } else if value > self.value().unwrap() {
                    self.right().find_node(value)
                    }
                else {
                    None
                }
            }, 
            None => None
        }
    }

    // relcalculate the height of a node
    fn recalc_height(&mut self) {
        let left = self.left();
        let right = self.right();
        self.set_height(1 + max(left.height(), right.height()));
    }

    // calculate the balance of a node with respect to its height
    fn get_balance(&self) -> isize {
        if self.is_none() {
            0
        } else {
            self.left().height() - self.right().height()
        }
    }

    // return the parent of a node
    fn parent(&self) -> AVLTreeNode<T> {
        match self {
            Some(tree_node) => tree_node.borrow().parent.clone(),
            None => None
        }
    }

    // return the left child of a node
    fn left(&self) -> AVLTreeNode<T> {
        match self {
            Some(tree_node) => tree_node.borrow().left.clone(),
            None => None
        }
    }

    // return the right child of a node
    fn right(&self) -> AVLTreeNode<T> {
        match self {
            Some(tree_node) => tree_node.borrow().right.clone(),
            None => None
        }
    }

    // return the parent of a node's parent
    fn grandparent(&self) -> AVLTreeNode<T> {
        self.parent().parent()
    }

    // return a parent node's sibling
    fn uncle(&self) -> AVLTreeNode<T> {
        if self.grandparent().left().is_none() || self.grandparent().right().is_none() {
            None
        } else if self.parent().compare(&self.grandparent().left()) {
            self.grandparent().right()
        } else {
            self.grandparent().left()
        }
    }

    // return a node's sibling
    fn sibling(&self) -> AVLTreeNode<T> {
        match self.compare(&self.parent().left()) {
            true => self.parent().right(),
            false => self.parent().left(),
        }
    }

    // return the height of a node
    fn height(&self) -> isize {
        match self {
            Some(tree_node) => tree_node.borrow().height,
            None => 0
        }
    }

    // set the height of a node
    fn set_height(&self, value: isize) {
        self.unwrapped().borrow_mut().height = value;
    }

    // set the parent of a node
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

    // set the left child of a node
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

    // set the right child of a node
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
#[derive(Clone, Debug, PartialEq)]
pub struct AVLTree<T> {
    root: AVLTreeNode<T>,
    len: usize
}

impl<T> AVLTree<T> {
    pub fn new() -> AVLTree<T> {
        AVLTree {
            root: None,
            len: 0
        }
    }
}

pub trait AVLTreeTraits<T> {
    // methods required to be implemented for a fully functional AVL tree
    fn search(&self, value: T) -> AVLTreeNode<T>;
    fn rotate(&mut self, node: &AVLTreeNode<T>, direction: bool);
    fn rebalance(&mut self, node: &mut AVLTreeNode<T>);
}

impl<T> TreeBase<T> for AVLTree<T> where T: Copy + PartialOrd + std::fmt::Debug {

    // get the height of a tree
    fn height(&self) -> usize {
        self.root.height() as usize
    }

    // check if a tree is empty
    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    // get the size of a tree
    fn size(&self) -> usize {
        self.len
    }

    // check if a tree contains a value
    fn contains(&self, value: T) -> bool {
        match self.search(value) {
            Some(_) => true,
            None => false
        }
    }

    // count the number of leaves in a tree
    fn count_leaves(&self) -> usize {
        self.root.count_leaves()
    }

    // insert a node into a tree
    fn insert_node(&mut self, value: T) {
        // REGULAR BINARY SEARCH TREE INSERTION 
        let mut new_node = AVLTreeNode::new(value);

        if self.is_empty() {
            self.root = new_node.clone();
            self.len += 1;
            return
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

        self.rebalance(&mut new_node);
        self.len += 1;
    }

    // remove/delete a node from a tree
    fn delete_node(&mut self, value: T) {
        let mut node = self.search(value);

        if node.is_none() {
            println!("Node does not exist!");
            return;
        }

        let mut unbalanced = node.left();
        if node.left().value().is_none() {
            unbalanced = match node.right().value() {
                None => node.parent(),
                Some(_) => node.right(),
            };
        }

        if node.left().value().is_some() && node.right().value().is_some() {
            let mut larger = node.left();

            while larger.right().value().is_some() {
                larger = larger.right();
            }

            node.set_value(larger.value().unwrap());
            node = larger.clone();
        }

        // set node to null sibling
        let mut child = match node.left().value() {
            Some(_) => node.left(),
            None => node.right()
        };

        if !node.compare(&self.root) && node.parent().is_some() {
            child.set_parent(node.parent());
            if node.compare(&node.parent().left()) {
                node.parent().set_left(child.clone());
            } else {
                node.parent().set_right(child.clone());
            }
        } else if child.is_none() {
            // empty tree if child is None
            self.root = None;
            self.len -= 1;
            return;
        } else {
            // set root to child
            self.root = child.clone();
            child.set_parent(None);
        }

        self.rebalance(&mut unbalanced);
        self.len -= 1;
    }

    // print the in-order traversal in a tree
    fn print(&self) {
        if self.is_empty() {
            println!("Tree is empty. Nothing to print.");
        } else {
            println!("Root: {:?}", self.root.value().unwrap());
            self.root.print_traversal();
            println!();
        }
    }

    // return all the values in an AVL tree by depth
    fn get_depth_vec(&self) -> Vec<(T, usize)> {
        self.root.get_depth_vec()
    }
}

impl<T> AVLTreeTraits<T> for AVLTree<T> where T: Copy + PartialOrd + std::fmt::Debug {

    // search for a node with given value in a tree
    fn search(&self, value: T) -> AVLTreeNode<T> {
        self.root.find_node(value)
    }

    // rotate a subtree/node either LEFT or RIGHT
    fn rotate(&mut self, node: &AVLTreeNode<T>, direction: bool) {
        let mut parent = node.parent().clone();
        let mut grandparent = node.grandparent().clone();
        let mut node = node.clone();

        if parent.compare(&self.root) {
            self.root = node.clone();
        } else {
            node.set_parent(grandparent.clone());
            if parent.compare(&grandparent.left()) {
                grandparent.set_left(node.clone());
            } else {
                grandparent.set_right(node.clone());
            }
        }

        parent.set_parent(node.clone());
        if direction == ROTATE_LEFT {
            if node.left().is_some() {
                node.left().set_parent(parent.clone());
            }
            parent.set_right(node.left());
            parent.recalc_height();
            node.set_left(parent);
        } else {
            if node.right().is_some() {
                node.right().set_parent(parent.clone());
            }
            parent.set_left(node.right());
            parent.recalc_height();
            node.set_right(parent);
        }

        node.recalc_height();
    }

    // rebalance a tree following insertion or deletion
    fn rebalance(&mut self, node: &mut AVLTreeNode<T>) {
        node.recalc_height();
        let balance = node.get_balance();

        // Left Cases
        if balance > 1 {
            // LEFT RIGHT
            if node.left().right().height() > node.left().left().height() {
                self.rotate(&node.left().right(), ROTATE_LEFT);
                self.rotate(&node.left(), ROTATE_RIGHT);
                self.root.recalc_height();
                return
            }
            // LEFT LEFT 
            else if node.left().right().height() <= node.left().left().height() {
                self.rotate(&node.left(), ROTATE_RIGHT);
                self.root.recalc_height();
                return;
            }
        }

        // RIGHT Cases
        if balance < -1 {
            // RIGHT LEFT
            if node.right().left().height() > node.right().right().height() {
                self.rotate(&node.right().left(), ROTATE_RIGHT);
                self.rotate(&node.right(), ROTATE_LEFT);
                self.root.recalc_height();
                return;
            }
            // RIGHT RIGHT
            else if node.right().left().height() <= node.right().right().height() {
                self.rotate(&node.right(), ROTATE_LEFT);
                self.root.recalc_height();
                return;
            }
        }

        if node.parent().is_some() && !node.compare(&self.root) {
            self.rebalance(&mut node.parent());
        }
    }
}
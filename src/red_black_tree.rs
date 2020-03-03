use std::cell::RefCell;
use std::rc::Rc;
use crate::tree::{TreeBase, NodeTraits};

static ROTATE_LEFT: bool = true;
static ROTATE_RIGHT: bool = false;

#[derive(Clone, Debug, PartialEq)]

/******************** TreeNode Helpers ********************/

pub enum NodeColor {
    Red,
    Black,
}

pub type RBTreeNode<T> = Option<Rc<RefCell<RBTNode<T>>>>;

#[derive(Debug)]
pub struct RBTNode<T> {
    color: NodeColor,
    value: Option<T>,
    parent: RBTreeNode<T>,
    left: RBTreeNode<T>,
    right: RBTreeNode<T>
}

pub trait RBTNodeTraits<T> {
    // helper functions
    fn new(val: T) -> RBTreeNode<T>;
    fn unwrapped(&self) -> Rc<RefCell<RBTNode<T>>>;
    fn compare(&self, node: &RBTreeNode<T>) -> bool;
    fn find_node(&self, value: T) -> RBTreeNode<T>;
    fn node_height(&self) -> usize;

    // RBT specific helper functions
    fn is_red(&self) -> bool;
    fn is_black(&self) -> bool;

    // getters for family members and node property
    fn parent(&self) -> RBTreeNode<T>;
    fn left(&self) -> RBTreeNode<T>; 
    fn right(&self) -> RBTreeNode<T>;
    fn grandparent(&self) -> RBTreeNode<T>;
    fn uncle(&self) -> RBTreeNode<T>;
    fn sibling(&self) -> RBTreeNode<T>;
    fn color(&self) -> NodeColor; // RBT specific node property

    // setters for node properties
    fn set_parent(&mut self, parent: RBTreeNode<T>);
    fn set_left(&mut self, left: RBTreeNode<T>);
    fn set_right(&mut self, right: RBTreeNode<T>);
    fn set_color(&self, color: NodeColor); // RBT specific node property
}

impl<T> NodeTraits<T> for RBTreeNode<T> where T: Copy + PartialOrd + std::fmt::Debug {

    // print in-order traversal of tree
    fn print_traversal(&self) {
        if self.is_some() && self.value().is_some() {
            self.left().print_traversal();
            if self.is_red() {
                print!("<{:?}>", self.value().unwrap());
            } else {
                print!("[{:?}]", self.value().unwrap());
            }
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
                // let value_in = self.value().unwrap();
                // if !vec.iter().any(|x| x.0 == value_in) {
                vec.push((self.value().unwrap(), depth));
                // }

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

impl<T> RBTNodeTraits<T> for RBTreeNode<T> where T: Copy + PartialOrd + std::fmt::Debug {

    // return new instance of a red-black tree node
    fn new(val: T) -> RBTreeNode<T> {
        let tree_node = Some(Rc::new(RefCell::new(RBTNode {
            color: NodeColor::Red,
            value: Some(val),
            parent: None,
            left: None,
            right: None
        })));
        tree_node.left().set_parent(tree_node.clone());
        tree_node.right().set_parent(tree_node.clone());

        tree_node
    }

    // return unwrapped tree node (reference counter)
    fn unwrapped(&self) -> Rc<RefCell<RBTNode<T>>> {
        match self {
            Some(tree_node) => Rc::clone(&tree_node),
            None => panic!("Error unwrapping tree node")
        }
    }

    // compare if two nodes are the same or not
    fn compare(&self, node: &RBTreeNode<T>) -> bool {
        if self.is_none() || node.is_none() {
            return false
        }
        Rc::ptr_eq(&self.unwrapped(), &node.unwrapped())
    }

    // search for a node with given value
    fn find_node(&self, value: T) -> RBTreeNode<T> {
        match self.value() {
            Some(_) => {
                if value == self.value().unwrap() {
                    self.clone()
                } else if value < self.value().unwrap() {
                    self.left().find_node(value)
                } else {
                    self.right().find_node(value)
                }
            },
            None => None
        }
    }

    // get the height of a node
    fn node_height(&self) -> usize {
        match self.value() {
            Some(_) => {
                let left_height = self.left().node_height();
                let right_height = self.right().node_height();

                if left_height > right_height {
                    left_height + 1
                } else {
                    right_height + 1
                }
            },
            None => 0
        }
    }

    // determine if a node is red
    fn is_red(&self) -> bool {
        self.color() == NodeColor::Red
    }

    // determine if a node is black
    fn is_black(&self) -> bool {
        self.color() == NodeColor::Black
    }

    // return the parent of a node
    fn parent(&self) -> RBTreeNode<T> {
        match self {
            Some(tree_node) => tree_node.borrow().parent.clone(),
            None => None
        }
    }

    // return the left child of a node
    fn left(&self) -> RBTreeNode<T> {
        match self {
            Some(tree_node) => tree_node.borrow().left.clone(),
            None => None
        }
    }

    // return the right child of a node
    fn right(&self) -> RBTreeNode<T> {
        match self {
            Some(tree_node) => tree_node.borrow().right.clone(),
            None => None
        }
    }

    // return the parent of a node's parent
    fn grandparent(&self) -> RBTreeNode<T> {
        self.parent().parent()
    }

    // return a parent node's sibling
    fn uncle(&self) -> RBTreeNode<T> {
        if self.grandparent().left().is_none() || self.grandparent().right().is_none() {
            None
        } else if self.parent().compare(&self.grandparent().left()) {
            self.grandparent().right()
        } else {
            self.grandparent().left()
        }
    }

    // return a node's sibling
    fn sibling(&self) -> RBTreeNode<T> {
        match self.compare(&self.parent().left()) {
            true => self.parent().right(),
            false => self.parent().left(),
        }
    }

    // return the color of a node
    fn color(&self) -> NodeColor {
        match self {
            Some(tree_node) => tree_node.borrow().color.clone(),
            None => NodeColor::Black // nil nodes are black
        }
    }

    // set the color of a node
    fn set_color(&self, color: NodeColor) {
        self.unwrapped().borrow_mut().color = color;
    }

    // set the parent of a node
    fn set_parent(&mut self, parent: RBTreeNode<T>) {
        match self {
            Some(tree_node) => tree_node.borrow_mut().parent = parent,
            None => *self = Some(Rc::new(RefCell::new(RBTNode {
                color: self.color(),
                value: self.value(),
                parent: parent,
                left: self.left(),
                right: self.right()
            })))
        }
    }

    // set the left child of a node
    fn set_left(&mut self, left: RBTreeNode<T>) {
        match self {
            Some(tree_node) => tree_node.borrow_mut().left = left,
            None => *self = Some(Rc::new(RefCell::new(RBTNode {
                color: self.color(),
                value: self.value(),
                parent: self.parent(),
                left: left,
                right: self.right()
            })))
        }
    }

    // set the right child of a node
    fn set_right(&mut self, right: RBTreeNode<T>) {
        match self {
            Some(tree_node) => tree_node.borrow_mut().right = right,
            None => *self = Some(Rc::new(RefCell::new(RBTNode {
                color: self.color(),
                value: self.value(),
                parent: self.parent(),
                left: self.left(),
                right: right
            })))
        }
    }
}

/******************** RBTree Helpers ********************/

pub struct RBTree<T> {
    root: RBTreeNode<T>,
    len: usize
}

impl<T> RBTree<T> {
    pub fn new() -> RBTree<T> {
        RBTree {
            root: None,
            len: 0
        }
    }
} 

pub trait RBTreeTraits<T> {
    // methods required to be implemented for a fully functional red-black tree
    fn search(&self, value: T) -> RBTreeNode<T>;
    fn rotate(&mut self, node: &RBTreeNode<T>, direction: bool);
    fn fix_insert_color(&mut self, node: &RBTreeNode<T>);
    fn fix_delete_color(&mut self, node: &RBTreeNode<T>);
}

impl<T> TreeBase<T> for RBTree<T> where T: Copy + PartialOrd + std::fmt::Debug {

    // get the height of a tree
    fn height(&self) -> usize {
        self.root.node_height()
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
        let mut new_node = RBTreeNode::new(value);

        if self.is_empty() {
            self.root = new_node.clone();
        } else if self.search(value).is_some() {
            // sticking with printing an err msg because panic causes the terminal to exit the program
            println!("Value already exists!");
            return;
        } else {
            let mut curr_node = self.root.clone();
            let mut curr_node_parent: RBTreeNode<T> = None;
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

        self.fix_insert_color(&new_node);
        self.len += 1;
    }

    // remove/delete a node from a tree
    fn delete_node(&mut self, value: T) {
        let mut node = self.search(value);

        if node.is_none() {
            println!("Node does not exist!");
            return;
        }

        if node.left().value().is_some() && node.right().value().is_some() {
            let mut larger = node.left();

            while larger.right().value().is_some() { // larger.left()
                larger = larger.right(); // larger.left()
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

        if node.is_black() {
            if child.is_red() {
                child.set_color(NodeColor::Black);
            } else {
                self.fix_delete_color(&child);
            }
        }

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

    // return all the values in a red-black tree by depth
    fn get_by_depth(&self) -> Vec<(T, usize)> {
        self.root.get_depth_vec()
    }
}

impl<T> RBTreeTraits<T> for RBTree<T> where T: Copy + PartialOrd + std::fmt::Debug {

    // search for a node with given value in a tree
    fn search(&self, value: T) -> RBTreeNode<T> {
        self.root.find_node(value)
    }

    // rotate a subtree/node either LEFT or RIGHT
    fn rotate(&mut self, node: &RBTreeNode<T>, direction: bool) {
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
            node.set_left(parent);
        } else {
            if node.right().is_some() {
                node.right().set_parent(parent.clone());
            }
            parent.set_left(node.right());
            node.set_right(parent);
        }
    }

    // rebalance a tree by fixing the colors of nodes after inserting a node
    fn fix_insert_color(&mut self, node: &RBTreeNode<T>) {
        // CASE 1
        if node.compare(&self.root) {
            node.set_color(NodeColor::Black);
            return;
        } 
        
        // CASE 2
        let parent = node.parent().clone();
        if parent.is_black() {
            return;
        }
        
        // CASE 3
        let uncle = node.uncle().clone();
        let grandparent = node.grandparent().clone();
        if uncle.is_some() && uncle.is_red() {
            parent.set_color(NodeColor::Black);
            uncle.set_color(NodeColor::Black);
            grandparent.set_color(NodeColor::Red);
            self.fix_insert_color(&grandparent);
            return;
        }
        
        // CASE 4
        let mut node = node.clone();
        if node.compare(&parent.right()) && parent.compare(&grandparent.left()) {
            self.rotate(&node, ROTATE_LEFT);
            node = node.left();
        } else if node.compare(&parent.left()) && parent.compare(&grandparent.right()) {
            self.rotate(&node, ROTATE_RIGHT);
            node = node.right();
        }

        // CASE 5
        let parent = node.parent().clone();
        let grandparent = node.grandparent().clone();
        if node.compare(&parent.left()) {
            self.rotate(&parent, ROTATE_RIGHT);
        } else {
            self.rotate(&parent, ROTATE_LEFT);
        }
        parent.set_color(NodeColor::Black);
        grandparent.set_color(NodeColor::Red);
    }

    // rebalance a tree by fixing the colors of nodes after deleting a node
    fn fix_delete_color(&mut self, node: &RBTreeNode<T>) {
        // CASE 1: node is root so done (it’s already black)
        if node.compare(&self.root) {
            return;
        }

        let mut sibling = node.sibling();

        // CASE 2
        if sibling.is_red() {
            sibling.parent().set_color(NodeColor::Red);
            sibling.set_color(NodeColor::Black);

            if node.compare(&node.parent().left()) {
                self.rotate(&sibling, ROTATE_LEFT);
            } else {
                self.rotate(&sibling, ROTATE_RIGHT);
            }

            // update sibling after rotating
            sibling = node.sibling();
        }

        // CASE 3
        if node.parent().is_black() && sibling.is_black() &&
            sibling.left().is_black() && sibling.right().is_black() {
                sibling.set_color(NodeColor::Red);
                self.fix_delete_color(&node.parent());
                return;
        }

        // CASE 4
        if node.parent().is_red() && sibling.is_black() &&
            sibling.left().is_black() && sibling.right().is_black() {
                sibling.set_color(NodeColor::Red);
                node.parent().set_color(NodeColor::Black);
                return;
        }

        // CASE 5
        if sibling.is_black() {
            sibling.set_color(NodeColor::Red);
            if node.compare(&node.parent().left()) &&
                sibling.left().is_red() && sibling.right().is_black() {
                    sibling.left().set_color(NodeColor::Black);
                    self.rotate(&sibling.left(), ROTATE_RIGHT);
            } else if node.compare(&node.parent().right()) &&
                sibling.left().is_black() && sibling.right().is_red() {
                    sibling.right().set_color(NodeColor::Black);
                    self.rotate(&sibling.left(), ROTATE_LEFT);
            }
            
            // update sibling
            sibling = node.sibling();
        }

        // CASE 6
        sibling.set_color(node.parent().color());
        node.parent().set_color(NodeColor::Black);
        if node.compare(&node.parent().left()) {
            sibling.right().set_color(NodeColor::Black);
            self.rotate(&sibling, ROTATE_LEFT);
        } else {
            sibling.left().set_color(NodeColor::Black);
            self.rotate(&sibling, ROTATE_RIGHT);
        }
    }
}
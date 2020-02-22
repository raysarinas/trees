use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]

/******************** TreeNode Helpers ********************/

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
    fn compare(&self, node: &TreeNode<T>) -> bool;
    fn node_height(&self) -> usize;
    fn print_traversal(&self);
    fn color(&self) -> NodeColor;
    fn value(&self) -> Option<T>;
    fn parent(&self) -> TreeNode<T>;
    fn left(&self) -> TreeNode<T>; 
    fn right(&self) -> TreeNode<T>;
    fn grandparent(&self) -> TreeNode<T>;
    fn uncle(&self) -> TreeNode<T>;
    fn set_color(&self, color: NodeColor);
    fn set_value(&self, value: T);
    fn set_parent(&mut self, parent: TreeNode<T>);
    fn set_left(&mut self, left: TreeNode<T>);
    fn set_right(&mut self, right: TreeNode<T>);
    fn find_node(&self, value: T) -> TreeNode<T>;
}

impl<T> NodeTraits<T> for TreeNode<T> where T: Copy + PartialOrd + std::fmt::Debug {
    fn new(val: T) -> TreeNode<T> {
        let tree_node = Some(Rc::new(RefCell::new(Node {
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

    fn unwrapped(&self) -> Rc<RefCell<Node<T>>> {
        match self {
            Some(tree_node) => Rc::clone(&tree_node),
            None => panic!("Error unwrapping tree node")
        }
    }

    fn compare(&self, node: &TreeNode<T>) -> bool {
        if self.is_none() || node.is_none() {
            return false
        }
        Rc::ptr_eq(&self.unwrapped(), &node.unwrapped())
    }

    fn node_height(&self) -> usize {
        match self {
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

    fn print_traversal(&self) {
        if self.is_some() && self.value().is_some() {
            self.left().print_traversal();
            if self.color() == NodeColor::Red {
                print!("<{:?}>", self.value().unwrap());
            } else {
                print!("[{:?}]", self.value().unwrap());
            }
            self.right().print_traversal();
        }
    }

    fn color(&self) -> NodeColor {
        match self {
            Some(tree_node) => tree_node.borrow().color.clone(),
            None => NodeColor::Black // nil nodes are black
        }
    }

    fn value(&self) -> Option<T> {
        match self {
            Some(tree_node) => tree_node.borrow().value,
            None => None
        }
    }

    fn parent(&self) -> TreeNode<T> {
        match self {
            Some(tree_node) => tree_node.borrow().parent.clone(),
            None => None
        }
    }

    fn left(&self) -> TreeNode<T> {
        match self {
            Some(tree_node) => tree_node.borrow().left.clone(),
            None => None
        }
    }

    fn right(&self) -> TreeNode<T> {
        match self {
            Some(tree_node) => tree_node.borrow().right.clone(),
            None => None
        }
    }

    fn grandparent(&self) -> TreeNode<T> {
        self.parent().parent()
    }

    fn uncle(&self) -> TreeNode<T> {
        if self.grandparent().left().is_none() || self.grandparent().right().is_none() {
            None
        } else if self.parent().compare(&self.grandparent().left()) {
            self.grandparent().right()
        } else {
            self.grandparent().left()
        }
    }

    fn set_color(&self, color: NodeColor) {
        self.unwrapped().borrow_mut().color = color;
    }

    fn set_value(&self, value: T) {
        self.unwrapped().borrow_mut().value = Some(value);
    }

    fn set_parent(&mut self, parent: TreeNode<T>) {
        match self {
            Some(tree_node) => tree_node.borrow_mut().parent = parent,
            None => *self = Some(Rc::new(RefCell::new(Node {
                color: self.color(),
                value: self.value(),
                parent: parent,
                left: self.left(),
                right: self.right()
            })))
        }
    }

    fn set_left(&mut self, left: TreeNode<T>) {
        match self {
            Some(tree_node) => tree_node.borrow_mut().left = left,
            None => *self = Some(Rc::new(RefCell::new(Node {
                color: self.color(),
                value: self.value(),
                parent: self.parent(),
                left: left,
                right: self.right()
            })))
        }
    }

    fn set_right(&mut self, right: TreeNode<T>) {
        match self {
            Some(tree_node) => tree_node.borrow_mut().right = right,
            None => *self = Some(Rc::new(RefCell::new(Node {
                color: self.color(),
                value: self.value(),
                parent: self.parent(),
                left: self.left(),
                right: right
            })))
        }
    }

    fn find_node(&self, value: T) -> TreeNode<T> {
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
}

/******************** RBTree Helpers ********************/

pub struct RBTree<T> {
    root: TreeNode<T>,
    len: usize
}

pub trait RBTreeTraits<T> {
    fn new() -> RBTree<T>;
    fn height(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
    fn search(&self, value: T) -> TreeNode<T>;
    fn rotate_left(&mut self, node: &TreeNode<T>);
    fn rotate_right(&mut self, node: &TreeNode<T>);
    fn insert_case5(&mut self, node: &TreeNode<T>);
    fn fix_insert_color(&mut self, node: &TreeNode<T>);
    fn fix_delete_color(&mut self, node: &TreeNode<T>);
    fn insert_node(&mut self, value: T);
    fn delete_node(&mut self, value: T);
    fn print(&self);
}

impl<T> RBTreeTraits<T> for RBTree<T> where T: Copy + PartialOrd + std::fmt::Debug {
    fn new() -> RBTree<T> {
        RBTree {
            root: None,
            len: 0
        }
    }

    // TODO ask miller if leaf nodes are included
    fn height(&self) -> usize {
        self.root.node_height()

        // use code below if leaf nodes are included
        // if self.len >= 1 {
        //     self.root.node_height() + 1
        // } else {
        //     0
        // }
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn size(&self) -> usize {
        self.len
    }

    // TODO: refactor because the code works but is bad lol
    fn search(&self, value: T) -> TreeNode<T> {

        if self.root.is_none() {
            return None
        } else if self.root.value().unwrap() == value {
            return self.root.clone()
        } else {
            self.root.find_node(value)
        }
    }

    fn rotate_left(&mut self, node: &TreeNode<T>) {
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

        if node.left().is_some() {
            node.left().set_parent(parent.clone());
        }

        parent.set_right(node.left().clone());
        node.set_left(parent.clone());
        parent.set_parent(node.clone());
    }

    fn rotate_right(&mut self, node: &TreeNode<T>) {
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

        if node.right().is_some() {
            node.right().set_parent(parent.clone());
        }

        parent.set_left(node.right().clone());
        node.set_right(parent.clone());
        parent.set_parent(node.clone());
    }

    fn insert_case5(&mut self, node: &TreeNode<T>) {
        let parent = node.parent().clone();
        let grandparent = node.grandparent().clone();

        if node.compare(&parent.left()) {
            self.rotate_right(&parent);
        } else {
            self.rotate_left(&parent);
        }

        parent.set_color(NodeColor::Black);
        grandparent.set_color(NodeColor::Red);
    }

    fn fix_insert_color(&mut self, node: &TreeNode<T>) {
        // case 1
        if node.compare(&self.root) {
            node.set_color(NodeColor::Black);
        // case 2
        } else if node.parent().color() == NodeColor::Black {
            return;
        // case 3
        } else if node.uncle().is_some() && node.uncle().color() == NodeColor::Red {
            node.parent().set_color(NodeColor::Black);
            node.uncle().set_color(NodeColor::Black);
            node.grandparent().set_color(NodeColor::Red);
            self.fix_insert_color(&node.grandparent());
        // case 4
        } else if node.uncle().color() == NodeColor::Black {    // uncle can be a nil node
            let parent = node.parent().clone();
            let grandparent = node.grandparent().clone();

            if node.compare(&parent.right()) && parent.compare(&grandparent.left()) {
                self.rotate_left(&node);
                self.insert_case5(&node.left());
            } else if node.compare(&parent.left()) && parent.compare(&grandparent.right()) {
                self.rotate_right(&node);
                self.insert_case5(&node.right());
            }
        }
    }

    fn insert_node(&mut self, value: T) {
        let mut new_node = TreeNode::new(value);

        if self.is_empty() {
            self.root = new_node.clone();
        } else if self.search(value).is_some() {
            println!("Value already exists!");
            return;
        } else {
            let mut curr_node = self.root.clone();
            let mut curr_node_parent: TreeNode<T> = None;
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

    // TODO
    fn fix_delete_color(&mut self, node: &TreeNode<T>) {

    }

    // TODO
    fn delete_node(&mut self, value: T) {
        // let mut node = self.search(value);

        // if node.is_none() {
        //     return;
        // }

        // if node.left().is_some() && node.right().is_some() {
        //     let mut larger = node.right();
        //     let larger_value = larger.unwrapped();

        //     // larger.set_value(node.unwrapped());

        // }

        // let old_node = node;
        // // set node to null sibling
        // if node.right().is_none() {
        //     node.set_value(node.left());
        // }
    }

    fn print(&self) {
        println!("Root: {:?}", self.root.value().unwrap());
        self.root.print_traversal();
        println!();
    }
}
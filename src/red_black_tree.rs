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
}

impl<T> NodeTraits<T> for TreeNode<T> where T: Copy {
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

    fn color(&self) -> NodeColor {
        match self {
            Some(tree_node) => tree_node.borrow().color.clone(),
            None => NodeColor::Red
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
        if self.parent().compare(&self.grandparent().left()) {
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
    fn contains(&self, value: T) -> bool;
    fn rotate_left(&self, node: TreeNode<T>);
    fn rotate_right(&self, node: TreeNode<T>);
    fn fix_insert_color(&self, node: &TreeNode<T>);
    fn fix_delete_color(&self, node: TreeNode<T>);
    fn insert_node(&mut self, value: T);
    fn delete_node(&self, value: T);
    fn in_order_traversal(&self, node: &TreeNode<T>);
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

    // TODO (extra thing i guess)
    fn contains(&self, value: T) -> bool {
        false
    }

    // TODO
    fn rotate_left(&self, node: TreeNode<T>) {
        let parent = node.parent();
        let grandparent = node.grandparent();
        let left = node.left();

    }

    // TODO
    fn rotate_right(&self, node: TreeNode<T>) {

    }

    // TODO
    fn fix_insert_color(&self, node: &TreeNode<T>) {
        // case 1
        if node.compare(&self.root) {
            node.set_color(NodeColor::Black);
        // case 2
        } else if node.parent().color() == NodeColor::Black {
            return;
        // case 3
        } else if node.parent().color() == NodeColor::Red && node.uncle().color() == NodeColor::Red {
            node.parent().set_color(NodeColor::Black);
            node.uncle().set_color(NodeColor::Black);
            node.grandparent().set_color(NodeColor::Red);
            self.fix_insert_color(&node.grandparent());
        // case 4
        } else if node.parent().color() == NodeColor::Red && node.uncle().color() == NodeColor::Black {
            // if node == node.parent().right() && node.parent() == node.grandparent().left() {
            //     rotate_left(node);
            // }
            // else if node == node.parent
            // do some more stuff
        } else {
            // error handling here yay
        }
    }

    fn insert_node(&mut self, value: T) {
        let mut new_node = TreeNode::new(value);

        if self.is_empty() {
            self.root = new_node.clone();
        } else if self.contains(value) {
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

        // self.fix_insert_color(&new_node);
        self.len += 1;
    }

    // TODO
    fn fix_delete_color(&self, node: TreeNode<T>) {

    }

    // TODO
    fn delete_node(&self, value: T) {

    }

    fn in_order_traversal(&self, node: &TreeNode<T>) {
        if node.is_some() && node.value().is_some() {
            self.in_order_traversal(&node.left());
            if node.color() == NodeColor::Red {
                print!("<{:?}>", node.value().unwrap());
            } else {
                print!("[{:?}]", node.value().unwrap());
            }
            self.in_order_traversal(&node.right());
        }
    }

    fn print(&self) {
        self.in_order_traversal(&self.root);
        println!();
    }
}
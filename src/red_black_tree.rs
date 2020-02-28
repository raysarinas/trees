use std::cell::RefCell;
use std::rc::Rc;

static ROTATE_LEFT: bool = true;
static ROTATE_RIGHT: bool = false;

#[derive(Clone, Debug, PartialEq)]

/******************** TreeNode Helpers ********************/

pub enum NodeColor {
    Red,
    Black,
}

pub type TreeNode<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T> {
    color: NodeColor,
    value: Option<T>,
    parent: TreeNode<T>,
    left: TreeNode<T>,
    right: TreeNode<T>
}

pub trait NodeTraits<T> {
    // helper functions
    fn new(val: T) -> TreeNode<T>;
    fn unwrapped(&self) -> Rc<RefCell<Node<T>>>;
    fn compare(&self, node: &TreeNode<T>) -> bool;
    fn find_node(&self, value: T) -> TreeNode<T>;
    fn node_height(&self) -> usize;
    fn print_traversal(&self);
    fn count_leaves(&self) -> usize;
    fn is_node_red(node: &TreeNode<T>) -> bool;

    // getters for node properties and family members
    fn color(&self) -> NodeColor;
    fn value(&self) -> Option<T>;
    fn parent(&self) -> TreeNode<T>;
    fn left(&self) -> TreeNode<T>; 
    fn right(&self) -> TreeNode<T>;
    fn grandparent(&self) -> TreeNode<T>;
    fn uncle(&self) -> TreeNode<T>;
    fn sibling(&self) -> TreeNode<T>;

    // setters for node properties
    fn set_color(&self, color: NodeColor);
    fn set_value(&self, value: T);
    fn set_parent(&mut self, parent: TreeNode<T>);
    fn set_left(&mut self, left: TreeNode<T>);
    fn set_right(&mut self, right: TreeNode<T>);
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

    fn find_node(&self, value: T) -> TreeNode<T> {
        match self.value() {
            Some(_) => {
                println!("FOUND SOME");
                println!("in value = {:?}", value);
                println!("self.value = {:?}", self.value().unwrap());
                if value == self.value().unwrap() {
                    println!("VALUE TO CLONE = {:?}", self.value());
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

    fn count_leaves(&self) -> usize {
        if self.is_none() {
            0
        } else if self.left().is_none() && self.right().is_none() {
            1
        } else {
            self.left().count_leaves() + self.right().count_leaves()
        }
    }

    fn is_node_red(node: &TreeNode<T>) -> bool {
        node.color() == NodeColor::Red
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

    fn sibling(&self) -> TreeNode<T> {
        match self.compare(&self.parent().left()) {
            true => self.parent().right(),
            false => self.parent().left(),
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
    fn search(&self, value: T) -> TreeNode<T>;
    fn rotate(&mut self, node: &TreeNode<T>, direction: bool);
    fn insert_case5(&mut self, node: &TreeNode<T>);
    fn fix_insert_color(&mut self, node: &TreeNode<T>);
    fn fix_delete_color(&mut self, node: &TreeNode<T>);
    fn fix_delete_color_2(&mut self, node: &TreeNode<T>);
    fn insert_node(&mut self, value: T);
    fn delete_node(&mut self, value: T);
    fn print(&self);
    fn count_leaves(&self) -> usize;
    fn get_higher_node(node: &TreeNode<T>) -> TreeNode<T>;
}

impl<T> RBTreeTraits<T> for RBTree<T> where T: Copy + PartialOrd + std::fmt::Debug {
    fn new() -> RBTree<T> {
        RBTree {
            root: None,
            len: 0
        }
    }

    // TODO ask miller if nil nodes are included
    fn height(&self) -> usize {
        self.root.node_height()

        // use code below if nil nodes are included
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

    fn search(&self, value: T) -> TreeNode<T> {
        self.root.find_node(value)
    }

    fn rotate(&mut self, node: &TreeNode<T>, direction: bool) {
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

    fn insert_case5(&mut self, node: &TreeNode<T>) {
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

    fn fix_insert_color(&mut self, node: &TreeNode<T>) {
        let parent = node.parent().clone();
        let uncle = node.uncle().clone();
        let grandparent = node.grandparent().clone();

        // case 1
        if node.compare(&self.root) {
            node.set_color(NodeColor::Black);
        // case 2
        } else if parent.color() == NodeColor::Black {
            return;
        // case 3
        } else if uncle.is_some() && uncle.color() == NodeColor::Red {
            parent.set_color(NodeColor::Black);
            uncle.set_color(NodeColor::Black);
            grandparent.set_color(NodeColor::Red);
            self.fix_insert_color(&grandparent);
        // case 4
        } else if uncle.color() == NodeColor::Black {    // uncle can be a nil node
            let mut node = node.clone();

            if node.compare(&parent.right()) && parent.compare(&grandparent.left()) {
                self.rotate(&node, ROTATE_LEFT);
                node = node.left();
            } else if node.compare(&parent.left()) && parent.compare(&grandparent.right()) {
                self.rotate(&node, ROTATE_RIGHT);
                node = node.right();
            }

            self.insert_case5(&node);
        }
    }

    fn insert_node(&mut self, value: T) {
        let mut new_node = TreeNode::new(value);

        if self.is_empty() {
            self.root = new_node.clone();
        } else if self.search(value).is_some() {
            // sticking with printing an err msg because panic causes the terminal to exit the program
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

    // TODO - fill out commented code and actually test
    fn fix_delete_color(&mut self, node: &TreeNode<T>) {
        println!("TRYING TO FIX DELETE COLOR");

        // case 1
        if node.parent().is_some() {
            println!("CASE 1 TRYING TO FIX DELETE COLOR");
            // find sibling
            let sibling = match node.compare(&node.parent().left()) {
                true => node.parent().right(),
                false => node.parent().left(),
            };

            // case 2
            if sibling.color() == NodeColor::Red {
                println!("CASE 2 TRYING TO FIX DELETE COLOR");
                sibling.parent().set_color(NodeColor::Red);
                sibling.set_color(NodeColor::Black);

                if node.compare(&node.parent().left()) {
                    self.rotate(&sibling, ROTATE_LEFT);
                } else {
                    self.rotate(&sibling, ROTATE_RIGHT);
                }

                // update sibling
                if node.compare(&node.parent().left()) {
                    sibling.set_value(node.parent().right().value().unwrap());
                    // sibling = node.parent().right()
                } else {
                    sibling.set_value(node.parent().left().value().unwrap());
                    // sibling = node.parent().left()
                }
            }

            if sibling.left().is_some() && sibling.right().is_some() {
                // case 3
                println!("CASE 3 TRYING TO FIX DELETE COLOR");
                // there is was a super long IF statement so i broke it into a nested one
                if node.parent().color() == NodeColor::Black && sibling.color() == NodeColor::Black {
                    if sibling.left().color() == NodeColor::Black && sibling.right().color() == NodeColor::Black {
                        sibling.set_color(NodeColor::Red);
                        self.fix_delete_color(&node.parent());
                    }
                }
                // case 4
                else if node.parent().color() == NodeColor::Red && sibling.color() == NodeColor::Black {
                    if sibling.left().color() == NodeColor::Black && sibling.right().color() == NodeColor::Black {
                        println!("CASE 4 TRYING TO FIX DELETE COLOR");
                        sibling.set_color(NodeColor::Red);
                        node.parent().set_color(NodeColor::Black);
                    }
                }
                // case 5
                else {
                   println!("CASE 5 TRYING TO FIX DELETE COLOR");
                   if sibling.color() == NodeColor::Black {
                       if node.compare(&node.parent().left()) {
                           if sibling.left().color() == NodeColor::Red && sibling.right().color() == NodeColor::Black {
                               sibling.set_color(NodeColor::Red);
                               sibling.left().set_color(NodeColor::Black);
                               self.rotate(&sibling.left(), ROTATE_RIGHT);
                               // rotate_right(sibling.left());
                           }
                       }
                       else if node.compare(&node.parent().right()) {
                        if sibling.left().color() == NodeColor::Black && sibling.right().color() == NodeColor::Red {
                            sibling.set_color(NodeColor::Red);
                            sibling.right().set_color(NodeColor::Black);
                            self.rotate(&sibling.left(), ROTATE_LEFT);
                            // rotate_left(sibling.left());
                        }
                    }

                    // update sibling
                    if node.compare(&node.parent().left()) {
                        sibling.set_value(node.parent().right().value().unwrap());
                        // sibling = node.parent().right();
                    } else {
                        sibling.set_value(node.parent().left().value().unwrap());
                        // sibling node.parent().left();
                    }
                   }


                   // case 6
                   println!("CASE 6 TRYING TO FIX DELETE COLOR");
                   sibling.set_color(node.parent().color());
                   node.parent().set_color(NodeColor::Black);
                   if node.compare(&node.parent().left()) {
                       sibling.right().set_color(NodeColor::Black);
                       self.rotate(&sibling, ROTATE_LEFT);
                       // rotate_left(sibling);
                   } else {
                       sibling.left().set_color(NodeColor::Black);
                       self.rotate(&sibling, ROTATE_RIGHT);
                       // rotate_right(sibling);
                   }

                }
            }
        }
    }

    fn fix_delete_color_2(&mut self, node: &TreeNode<T>) {
        // case 1: node is root so done (it’s already black)
        if node.compare(&self.root) {
            return;
        }

        let mut sibling = node.sibling();

        // Case 2, sibling of node is red:
        // Set parent’s color to red, and sibling’s color to black 
        // Rotate left/right on sibling if node is a lChild/rChild
        // Update sibling to node’s new sibling
        if node.parent().is_some() {
            println!("case 2 delete");
            // actually start case 2 here:
            if sibling.color() == NodeColor::Red {

                sibling.parent().set_color(NodeColor::Red);
                sibling.set_color(NodeColor::Black);

                if node.compare(&node.parent().left()) {
                    println!("LEFT ROTATING");
                    self.rotate(&sibling, ROTATE_LEFT);
                } else {
                    println!("RIGHT ROTATING");
                    self.rotate(&sibling, ROTATE_RIGHT);
                }

                // update sibling after rotating
                sibling = node.sibling();

            }
        }

        // CASE 3
        if node.parent().color() == NodeColor::Black && sibling.color() == NodeColor::Black {
            if sibling.left().color() == NodeColor::Black && sibling.right().color() == NodeColor::Black {
                println!("DELETE CASE 3");
                sibling.set_color(NodeColor::Red);
                self.fix_delete_color_2(&node.parent());
                return;
            }
        }
        // CASE 4
        if node.parent().color() == NodeColor::Red && sibling.color() == NodeColor::Black {
            if sibling.left().color() == NodeColor::Black && sibling.right().color() == NodeColor::Black {
                println!("DELETE CASE 4");
                sibling.set_color(NodeColor::Red);
                node.parent().set_color(NodeColor::Black);
                return;
            }
        }

        // CASE 5
        if sibling.color() == NodeColor::Black {
            println!("DELETE CASE 5");
            sibling.set_color(NodeColor::Red);
            if node.compare(&node.parent().left()) {
                if sibling.left().color() == NodeColor::Red && sibling.right().color() == NodeColor::Black {
                    println!("DELETE CASE 5 - RIGHT ROTATE");
                    sibling.left().set_color(NodeColor::Black);
                    self.rotate(&sibling.left(), ROTATE_RIGHT);
                    // rotate_right(sibling.left());
                }
            }
            else if node.compare(&node.parent().right()) {
             if sibling.left().color() == NodeColor::Black && sibling.right().color() == NodeColor::Red {
                println!("DELETE CASE 5 - LEFT ROTATE"); 
                sibling.right().set_color(NodeColor::Black);
                self.rotate(&sibling.left(), ROTATE_LEFT);
                 // rotate_left(sibling.left());
             }
         }

         // update sibling
         sibling = node.sibling();
        }

        // CASE 6
        println!("DELETE CASE 6");
        sibling.set_color(node.parent().color());
        node.parent().set_color(NodeColor::Black);

        if node.compare(&node.parent().left()) {
            sibling.right().set_color(NodeColor::Black);
            self.rotate(&sibling, ROTATE_LEFT);
            // rotate_left(sibling);
        } else {
            sibling.left().set_color(NodeColor::Black);
            self.rotate(&sibling, ROTATE_RIGHT);
            // rotate_right(sibling);
        }

    }

    fn get_higher_node(node: &TreeNode<T>) -> TreeNode<T> {
        println!("in get_higher_node");
        if node.right().is_none() {
            node.clone()
        } else {
            return Self::get_higher_node(&node.right())
        }
    }

    fn delete_node(&mut self, value: T) {
        let mut node = self.search(value);

        if node.is_none() {
            return;
        }

        if node.left().value().is_some() && node.right().value().is_some() {
            let mut larger = node.left(); // node.right()
            println!("init larger = {:?}", larger.value());
            let mut temp = larger.clone();
            while larger.right().value().is_some() { // larger.left()
                println!("======> larger in loop is = {:?}", larger.value());
                larger = larger.right(); // larger.left()
                temp = larger.parent();
            }
            // larger = larger.parent();
            // larger = Self::get_higher_node(&node.left());
            // println!("USING GET LARGER FUNC LARGER ==== {:?} ", larger.value());

            println!("larger is now = {:?}", larger.value());
            println!(">>>>>>>>>>>>>>>>> temp is now = {:?}", temp.value());
            // larger.set_value(temp.value().unwrap());
            println!(">>>>>>>>>>>>>>>>> LARGER IS NOW TEMP = {:?}", larger.value());
            // let mut larger = Self::get_higher_node(&node.left());
            node.set_value(larger.value().unwrap());

            println!("node value is now = {:?}", node.value());
            // mem::swap(&mut node, &mut larger);
            // let temp = mem::replace(&mut node, larger);
            // println!("larger after swapping with node = {:?}", temp.value());

            // println!("temp = {:?}", temp);
            // node = temp;
            node = larger.clone();
            println!("node is now = {:?}", node.value());

            // if value == 5 {
            //     process::exit(0);
            // }

        }

        // set node to null sibling
        
        println!("setting node to null sibling now");
        println!("====== node is now = {:?}", node.value());
        let mut child = match node.left() {
            Some(_) => node.right(),
            None => node.left()
        };

        println!("child is now set to = {:?}", child.value());

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
        } else {
            // set root to child
            self.root = child.clone();
            child.set_parent(None);
        }

        if node.color() == NodeColor::Black {
            if child.color() == NodeColor::Red {
                println!("dont need to fix color");
                child.set_color(NodeColor::Black);
            } else {
                println!("NEED TO FIX COLOR");
                self.fix_delete_color_2(&child);
            }
        }

        self.len -= 1;
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
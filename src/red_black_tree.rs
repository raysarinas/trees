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
            None => None
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
    fn fix_insert_color(&self, node: TreeNode<T>);
    fn fix_delete_color(&self, node: TreeNode<T>);
    fn insert_node(&mut self, value: T);
    fn delete_node(&self, value: T);
    fn print(&self);
}

impl<T> RBTreeTraits<T> for RBTree<T> where T: Copy + PartialOrd {
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

    // TODO (extra thing i guess)
    fn contains(&self, value: T) -> bool {
        false
    }

    // TODO
    fn rotate_left(&self, node: TreeNode<T>) {

    }

    // TODO
    fn rotate_right(&self, node: TreeNode<T>) {

    }

    // TODO
    fn fix_insert_color(&self, node: TreeNode<T>) {

    }

    fn insert_node(&mut self, value: T) {
        let new_node = TreeNode::new(value);

        if self.root.is_none() {
            self.root = TreeNode::new(value);
        } else if self.contains(value) {
            println!("Value already exists!");
            return;
        } else {
            let mut curr_node = self.root.clone();
            let mut is_left_child = true;
            
            while curr_node.value().is_some() {
                if value < curr_node.value().unwrap() {
                    curr_node = curr_node.left();
                    is_left_child = true;
                } else {
                    curr_node = curr_node.right();
                    is_left_child = false;
                }
            }
            
            new_node.set_parent(curr_node.parent());
            if is_left_child {
                curr_node.parent().set_left(new_node.clone());
            } else {
                curr_node.parent().set_right(new_node.clone());
            }
        }

        self.fix_insert_color(new_node.clone());
        self.len += 1;
    }

    // TODO
    fn fix_delete_color(&self, node: TreeNode<T>) {

    }

    // TODO
    fn delete_node(&self, value: T) {

    }

    // TODO
    fn print(&self) {

    }
}
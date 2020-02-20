use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]

enum NodeColor {
    Red,
    Black,
}

// type Tree = Rc<RefCell<TreeNode<u32>>>;
// type RedBlackTree= Option<Tree>;

struct TreeNode<T> {
    pub color: NodeColor,
    pub value: Option<T>,
    pub parent: Option<Rc<RefCell<TreeNode<T>>>>,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>
}

impl<T> TreeNode<T> {
    fn new(val: T) -> TreeNode<T> {
        TreeNode {
            color: NodeColor::Black,
            value: Some(val),
            parent: None,
            left: None,
            right: None
        }
    }

    fn is_some(& self) -> bool {
        match self.value {
            Some(_) => true,
            None => false,
        }
    }

    fn new_wrapped(val: T) -> Option<Rc<RefCell<TreeNode<T>>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    // fn unwrap(node: Option<Rc<RefCell<TreeNode<T>>>>) -> TreeNode<T> {
    //     node.as_ref().unwrap().as_ref().get_mut()
    // }
}

struct RBTree<T> {
    root: Option<Rc<RefCell<TreeNode<T>>>>,
    len: usize
}

impl<T> RBTree <T>
    where T: Ord + PartialEq + PartialOrd {
    // fn rotate() {

    // }
    
    // fn fixDelColor() {
    
    // }

    fn fixInsColor() {
    
    }
    
    pub fn delete_node(&mut self) {
        self.len -= 1;
    }

    pub fn search(&mut self, value: T) -> Option<Rc<RefCell<TreeNode<T>>>> {
        let mut curr_node = self.root;

        while !curr_node.is_none() {
            let mut unwrapped_curr_node = curr_node.unwrap().borrow();

            if value < unwrapped_curr_node.value.unwrap() {
                curr_node = unwrapped_curr_node.left;
            } else if value > unwrapped_curr_node.value.unwrap() {
                curr_node = unwrapped_curr_node.right;
            } else {
                // if value == curr node
                return curr_node;
            }
        }
        None
    }

    pub fn delete_node_2(&mut self, value: T) {

        let node_to_rm = self.search(value);

        if node_to_rm.is_none() {
            return
        }

        self.len -= 1;

        let unwrapped_node = node_to_rm.unwrap().borrow();
        if unwrapped_node.left.is_some() && unwrapped_node.right.is_some() {
            
        }

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
        if self.len == 0 {
            self.root = TreeNode::new_wrapped(value);
        } else {
            let root_unwrapped = self.root.unwrap().borrow();
            let mut new_node = TreeNode::new(value);
            let mut subroot = root_unwrapped;
            let is_left_child = true;

            while new_node.value != root_unwrapped.value && new_node.parent.is_none() {

                if !subroot.is_some() {

                    // insert at empty node
                    new_node.parent = subroot.parent;
                    let mut unwrapped_parent = subroot.parent.unwrap().borrow();

                    if is_left_child {
                        unwrapped_parent.left = TreeNode::new_wrapped(value);
                    } else {
                        unwrapped_parent.right = TreeNode::new_wrapped(value);
                    }

                } else if value < subroot.value.unwrap() {
                    subroot = subroot.left.unwrap().borrow();
                    is_left_child = true;
                    
                } else { //value > curr_node_unwrapped.value {
                    subroot = subroot.right.unwrap().borrow();
                    is_left_child = false;
                }
            }

            // rebalance the tree here i guess and fix colors
            fixInsColor();
        }
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
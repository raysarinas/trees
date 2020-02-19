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
    pub value: T,
    pub parent: Option<Rc<RefCell<TreeNode<T>>>>,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>
}

impl<T> TreeNode<T> {
    fn new(val: T) -> TreeNode<T> {
        TreeNode {
            color: NodeColor::Red,
            value: val,
            parent: None,
            left: None,
            right: None
        }
    }

    // fn unwrap(node: Option<Rc<RefCell<TreeNode<T>>>>) -> &'a mut TreeNode<T> {
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

    // fn fixInsColor() {
    
    // }
    
    pub fn delete_node(&mut self) {
        self.len -= 1;
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


    pub fn insert_node(&mut self, value: T) {
        if self.len == 0 {
            self.root = Some(Rc::new(RefCell::new(TreeNode::new(value))));
        } else {
            let curr_node = &self.root;
            let curr_node_unwrapped = curr_node.as_ref().unwrap().as_ref().get_mut();

            if value == curr_node_unwrapped.value {
                return
            }

            // loop  {}
            let curr_node = if value < curr_node_unwrapped.value {
                curr_node_unwrapped.left
            } else {
                curr_node_unwrapped.right
            };

            let new_node = TreeNode::new(value);
            new_node.parent = curr_node_unwrapped.parent;

            match curr_node {
                // _ => Some(Rc::new(RefCell::new(new_node))),
                Some(sub_node) => self.insert_node(value),
                None => {
                    // let temp = Some(Rc::new(RefCell::new(TreeNode::new(value))));
                    // *curr_node = temp;
                    *curr_node = Some(Rc::new(RefCell::new(new_node)));
                },
                // &mut Some(ref mut sub_node) => sub_node.insert_node(value),
                // &mut None => {
                //     let temp = TreeNode { value: value, left_child: None, right_child: None};
                //     let boxed_node = Some(Box::new(temp));
                //     *new_node = boxed_node;
                // }
            }
        }

        self.len += 1;
    }


    /*
        public void insert(int value) {
    	Node currentNode = root;
    	
    	while ( currentNode != null && currentNode.value != null ) {
    		if ( value < currentNode.value ) {
    			currentNode = currentNode.lChild;
    		}
    		else if ( value >= currentNode.value ) {
    			currentNode = currentNode.rChild;
    		}
//    		else {
////    			return;
//    		}
    	}
    	
    	Node newNode = new Node( value );
    	if ( root == null ) {
    		root = newNode;
    	}
    	else {
    		newNode.parent = currentNode.parent;
    		if ( currentNode.parent.lChild == currentNode ) {
    			newNode.parent.lChild = newNode;
    		}
    		else {
    			newNode.parent.rChild = newNode;
    		}
    	}
    	newNode.color = Node.RED;
    	fixInsColor( newNode );
    	size++;
    }
    */
    
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn print() {
    
    }

    pub fn size(&self) -> usize {
        self.len
    }
}
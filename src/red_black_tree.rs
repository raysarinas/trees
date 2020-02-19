use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]

enum NodeColor {
    Red,
    Black,
}

type Tree = Rc<RefCell<TreeNode<u32>>>;
type RedBlackTree= Option<Tree>;

struct TreeNode<T> {
    pub color: NodeColor,
    pub value: T,
    pub parent: RedBlackTree,
    left: RedBlackTree,
    right: RedBlackTree,
}

struct RBTree<T> {
    root: TreeNode<T>,
    len: usize
}

impl<T: PartialEq + PartialOrd> RBTree<T> {
    // fn rotate() {

    // }
    
    // fn fixDelColor() {
    
    // }

    // fn fixInsColor() {
    
    // }
    
    pub fn delete_node(&mut self) {
        self.len -= 1;
    }
    
    pub fn height() {
    
    }

    pub fn insert_node(&mut self, value: T) {
        self.len += 1;

        let curr_node = &self.root;

        if value == curr_node.value {
            return
        }

        // let new_node = if value < curr_node.value {
        // }

        // new_node = TreeNode { }
        //let new_node = if value < self.value { &mut self.left_child } else { &mut self.right_child };

        match new_node {
            &mut Some(ref mut sub_node) => sub_node.insert_node(value),
            &mut None => {
                let temp = TreeNode { value: value, left_child: None, right_child: None};
                let boxed_node = Some(Box::new(temp));
                *new_node = boxed_node;
            }
        }
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
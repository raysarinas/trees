mod avl_tree;
mod red_black_tree;

use red_black_tree::*;

fn main() {
    // Test TreeNode
    let treenode: red_black_tree::TreeNode<u32> = red_black_tree::TreeNode::new(5);
    
    println!("{:?}", treenode.value());
    println!("{:?}", treenode.color());

    treenode.set_color(red_black_tree::NodeColor::Black);
    println!("{:?}", treenode.color());

    treenode.set_value(10);
    println!("{:?}", treenode.value());


    // Test RBTree
    let mut rbt: red_black_tree::RBTree<u32> = red_black_tree::RBTree::new();
    rbt.insert_node(5);
    println!("{}", rbt.size());
    // won't know for sure if the following will work unless fix_insert_color is done
    rbt.insert_node(70);
    println!("{}", rbt.size());
    rbt.insert_node(35);
    println!("{}", rbt.size());
}

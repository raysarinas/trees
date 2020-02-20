mod avl_tree;
mod red_black_tree;

use red_black_tree::*;

fn main() {
    // Test TreeNode
    let treenode: red_black_tree::TreeNode<u32> = red_black_tree::TreeNode::new(5);
    
    println!("{:?}", treenode.value());
    println!("{:?}", treenode.color());

    treenode.set_color(red_black_tree::NodeColor::Red);
    println!("{:?}", treenode.color());

    treenode.set_value(10);
    println!("{:?}", treenode.value());


    // Test RBTree
    let rbt: red_black_tree::RBTree<u32> = red_black_tree::RBTree::new();
    // rbt.insert_node2(5);
    // println!("{}", rbt.size());
}

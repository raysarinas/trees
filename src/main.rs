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
    println!("\n==== Start Testing RBTree Here ====\n");
    let mut rbt: red_black_tree::RBTree<u32> = red_black_tree::RBTree::new();
    println!("empty height = {}", rbt.height());

    rbt.insert_node(5);
    println!("size = {}", rbt.size());
    rbt.print();
    println!("height with 1 node = {}", rbt.height());

    // HEIGHTS ARE OFF BY ONE AND IDK WHY YET
    rbt.insert_node(70);
    println!("size = {}", rbt.size());
    rbt.print();
    println!("height = {}", rbt.height());
    
    rbt.insert_node(35);
    println!("size = {}", rbt.size());
    rbt.print();
    println!("height = {}", rbt.height());

    rbt.insert_node(8);
    println!("size = {}", rbt.size());
    rbt.print();
    println!("height = {}", rbt.height());

    rbt.insert_node(100);
    println!("size = {}", rbt.size());
    rbt.print();
    println!("height = {}", rbt.height());

    rbt.insert_node(60);
    println!("size = {}", rbt.size());
    rbt.print();
    println!("height = {}", rbt.height());
    
}

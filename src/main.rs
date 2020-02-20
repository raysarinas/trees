mod avl_tree;
mod red_black_tree;

fn main() {
    // let mut rbt: red_black_tree::RBTree<u32> = red_black_tree::RBTree::new();
    // rbt.insert_node2(5);
    // println!("{}", rbt.size());

    let mut treenode: red_black_tree::TreeNode<u32> = red_black_tree::TreeNode::new(5);
    println!("{:?}", treenode.va());
}

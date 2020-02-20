mod avl_tree;
// mod red_black_tree;
mod rbt_raymond;

fn main() {
    let mut rbt: rbt_raymond::RBTree<u32> = rbt_raymond::RBTree::new(5);
    // rbt.insert_node2(5);
    println!("{}", rbt.size());
    println!("{:?}", rbt);
}

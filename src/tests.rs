use super::*;

// #[cfg(test)]
// TODO: write actual tests lmao
// moved the stuff here to write the command line interface in main
// #[test]
pub fn it_works() {
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
    println!("no leaves = {}", rbt.count_leaves());
    println!();

    println!("Inserting 5 ...");
    rbt.insert_node(5);
    println!("size = {}", rbt.size());
    rbt.print();
    println!("height with 1 node = {}", rbt.height());
    println!("1 node = 1 leaf? = {}", rbt.count_leaves());
    println!();

    // HEIGHTS ARE OFF BY ONE AND IDK WHY YET
    println!("Inserting 70 ...");
    rbt.insert_node(70);
    println!("size = {}", rbt.size());
    rbt.print();
    println!("height = {}", rbt.height());
    println!();

    println!("Inserting 35 ...");
    rbt.insert_node(35);
    println!("size = {}", rbt.size());
    rbt.print();
    println!("height = {}", rbt.height());
    println!("3 nodes = 2 leaves = {}", rbt.count_leaves());
    println!();

    println!("Inserting 8 ...");
    rbt.insert_node(8);
    println!("size = {}", rbt.size());
    rbt.print();
    println!("height = {}", rbt.height());
    println!("num leaves = {}", rbt.count_leaves());
    println!();

    println!("Inserting 100 ...");
    rbt.insert_node(100);
    println!("size = {}", rbt.size());
    rbt.print();
    println!("height = {}", rbt.height());
    println!("num leaves = {}", rbt.count_leaves());
    println!();

    println!("Inserting 60 ...");
    rbt.insert_node(60);
    println!("size = {}", rbt.size());
    rbt.print();
    println!("height = {}", rbt.height());
    println!("num leaves = {}", rbt.count_leaves());
    println!();

    println!("Inserting 120 ...");
    rbt.insert_node(120);
    println!("size = {}", rbt.size());
    rbt.print();
    println!("height = {}", rbt.height());
    println!("num leaves = {}", rbt.count_leaves());
    println!();

    println!("Inserting 1 ...");
    rbt.insert_node(1);
    println!("size = {}", rbt.size());
    rbt.print();
    println!("height = {}", rbt.height());
    println!("num leaves = {}", rbt.count_leaves());
    println!();

    println!("Inserting 84 ...");
    rbt.insert_node(84);
    println!("size = {}", rbt.size());
    rbt.print();
    println!("height = {}", rbt.height());
    println!("num leaves = {}", rbt.count_leaves());
    println!();

    println!("Inserting 17 ...");
    rbt.insert_node(17);
    println!("size = {}", rbt.size());
    rbt.print();
    println!("height = {}", rbt.height());
    println!("num leaves = {}", rbt.count_leaves());
    println!();

    assert!(rbt.search(35).is_some());
    assert!(rbt.search(5).is_some());
    assert!(rbt.search(1).is_some());
    assert!(rbt.search(8).is_some());
    assert!(rbt.search(17).is_some());
    assert!(rbt.search(60).is_some());
    assert!(rbt.search(70).is_some());
    assert!(rbt.search(84).is_some());
    assert!(rbt.search(100).is_some());
    assert!(rbt.search(120).is_some());
    assert!(rbt.search(10).is_none());

    // println!("Deleting 1 ...");
    // rbt.delete_node(1);
    // println!("size = {}", rbt.size());
    // rbt.print();
    // println!("height = {}", rbt.height());
    // println!("num leaves = {}", rbt.count_leaves());
    // println!();

    println!("\n==== Start Testing DELETE RBTree Here ====\n");
    let mut rbt2: red_black_tree::RBTree<u32> = red_black_tree::RBTree::new();

    println!("Inserting 30 ...");
    rbt2.insert_node(30);
    println!("size = {}", rbt2.size());
    rbt2.print();
    println!("height = {}", rbt2.height());
    println!("leaves = {}", rbt2.count_leaves());
    println!();

    println!("Inserting 20 ...");
    rbt2.insert_node(20);
    println!("size = {}", rbt2.size());
    rbt2.print();
    println!("height = {}", rbt2.height());
    println!("leaves = {}", rbt2.count_leaves());
    println!();

    println!("Inserting 40 ...");
    rbt2.insert_node(40);
    println!("size = {}", rbt2.size());
    rbt2.print();
    println!("height = {}", rbt2.height());
    println!("leaves = {}", rbt2.count_leaves());
    println!();

    println!("Inserting 10 ...");
    rbt2.insert_node(10);
    println!("size = {}", rbt2.size());
    rbt2.print();
    println!("height = {}", rbt2.height());
    println!("leaves = {}", rbt2.count_leaves());
    println!();

    println!("Deleting 10 ...");
    rbt2.delete_node(10);
    println!("size = {}", rbt2.size());
    rbt2.print();
    println!("height = {}", rbt2.height());
    println!("num leaves = {}", rbt2.count_leaves());
    println!();

    println!("Inserting 50 ...");
    rbt2.insert_node(50);
    println!("size = {}", rbt2.size());
    rbt2.print();
    println!("height = {}", rbt2.height());
    println!("leaves = {}", rbt2.count_leaves());
    println!();

    println!("Deleting 20 ...");
    rbt2.delete_node(20);
    println!("size = {}", rbt2.size());
    rbt2.print();
    println!("height = {}", rbt2.height());
    println!("num leaves = {}", rbt2.count_leaves());
    println!();

    println!("Deleting 30 ...");
    rbt2.delete_node(30);
    println!("size = {}", rbt2.size());
    rbt2.print();
    println!("height = {}", rbt2.height());
    println!("num leaves = {}", rbt2.count_leaves());
    println!();

    


}

pub fn delete_cases_1_3_5_6() {
    let mut rbt: red_black_tree::RBTree<u32> = red_black_tree::RBTree::new();
    
    for x in 1..=10 {
        rbt.insert_node(x);
        println!("inserting ... {}", x);
    }


    rbt.delete_node(4);
    // rbt.print();
    rbt.delete_node(6);
    rbt.print();
    rbt.delete_node(8);
    // rbt.print();
    rbt.delete_node(1);
    rbt.print();
    // println!("-----------------------------PRINTING DELETING 5");
    rbt.delete_node(5);
    rbt.print();
    rbt.delete_node(9);
    rbt.delete_node(2);
    rbt.print();

}

pub fn delete_cases_2_4() {
    println!("\n==== Start Testing DELETE RBTree Here ====\n");

    let mut rbt: red_black_tree::RBTree<u32> = red_black_tree::RBTree::new();
    let mut vec_in: Vec<u32> = vec![5, 70, 35, 8, 98, 60, 99, 99, 1, 84, 17];

    for &x in vec_in.iter() {
        println!("inserting {} ...", x);
        rbt.insert_node(x);
        println!("size is now = {}", rbt.size());
        rbt.print();
        println!("height = {}", rbt.height());
        println!("leaves = {}", rbt.count_leaves());
        println!();
    }

    vec_in = vec![17, 84, 99, 5, 1, 60];
    for &x in vec_in.iter() {
        println!("deleting {} ...", x);
        rbt.delete_node(x);
        println!("size is now = {}", rbt.size());
        rbt.print();
        println!("height = {}", rbt.height());
        println!("leaves = {}", rbt.count_leaves());
        println!();
    }
 
 
 }

 pub fn avl_test() {
    let mut avl: avl_tree::AVLTree<u32> = avl_tree::AVLTree::new();
 }
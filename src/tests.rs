use super::*;
use avl_tree::*;
use red_black_tree::*;
use crate::tree::*;
// #[cfg(test)]
// TODO: write actual tests lmao
// moved the stuff here to write the command line interface in main
// #[test]
pub fn benchmark_redblack() {
    for tree_size in vec![10_000, 40_000, 70_000, 100_000, 130_000] {
        let mut tree: red_black_tree::RBTree<u32> = red_black_tree::RBTree::new();

        for i in 0..tree_size {
            tree.insert_node(i);
        }

        let depth = tree.get_by_depth();
        let time = std::time::Instant::now();

        for i in 0..tree_size/10 {
            match tree.contains(depth[i as usize].value.unwrap()) {
                true => { continue; },
                false => println!("nope"),
            }
        }

        println!("Elapsed time for {}: {} ms", tree_size, time.elapsed().as_millis());
    }
}

pub fn benchmark_avl() {
    for tree_size in vec![10_000, 40_000, 70_000, 100_000, 130_000] {
        let mut tree: avl_tree::AVLTree<u32> = avl_tree::AVLTree::new();

        for i in 0..tree_size {
            tree.insert_node(i);
        }

        let depth = tree.get_depth_vec();
        let time = std::time::Instant::now();
        

        for i in 0..tree_size/10 {
            match tree.contains(depth[i as usize].value.unwrap()) {
                true => { continue; },
                false => println!("nope"),
            }
        }

        println!("Elapsed time for {}: {} ms", tree_size, time.elapsed().as_millis());
    }
}

pub fn it_works() {
    // Test TreeNode
    let treenode: red_black_tree::RBTreeNode<u32> = red_black_tree::RBTreeNode::new(5);
        
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

    assert!(rbt.contains(35) == true);
    assert!(rbt.contains(5) == true);
    assert!(rbt.contains(1) == true);
    assert!(rbt.contains(8) == true);
    assert!(rbt.contains(17) == true);
    assert!(rbt.contains(60) == true);
    assert!(rbt.contains(70) == true);
    assert!(rbt.contains(84) == true);
    assert!(rbt.contains(100) == true);
    assert!(rbt.contains(120) == true);
    assert!(rbt.contains(10) == false);

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

pub fn test_delete_rbt() {
    let mut rbt: red_black_tree::RBTree<u32> = red_black_tree::RBTree::new();

    for x in 1..=10 {
        rbt.insert_node(x);
        println!("inserting ... {}", x);
    }

    assert!(rbt.is_empty() == false);
    rbt.print();

    println!("deleting everything ..");

    rbt.delete_node(6);
    // rbt.print();
    rbt.delete_node(4);
    // rbt.print();
    rbt.delete_node(2);
    // rbt.print();
    rbt.delete_node(8);
    rbt.delete_node(9);
    rbt.delete_node(7);
    rbt.delete_node(3);
    rbt.delete_node(1);
    rbt.delete_node(5);
    rbt.delete_node(10);
    assert!(rbt.is_empty() == true);
    rbt.print();
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
    // let mut avl: avl_tree::AVLTree<u32> = avl_tree::AVLTree::new();
        //let mut avl: avl_tree::AVLTree<u32> = avl_tree::AVLTree::new();

        println!("\n==== Start Testing AVLTree Here ====\n");

        let mut avl: avl_tree::AVLTree<u32> = avl_tree::AVLTree::new();
        // let mut vec_in: Vec<u32> = vec![5, 70, 35]; // right left //, 8, 98, 60]; // poop on 99];//, 8, 98, 60, 99, 99, 1, 84, 17];
        // let mut vec_in: Vec<u32> = vec![5, 3, 1]; // left left
        // let mut vec_in: Vec<u32> = vec![5, 1, 3]; // left right
        // let mut vec_in: Vec<u32> = vec![5, 70, 90]; // right right
        let mut vec_in: Vec<u32> = vec![5, 70, 35, 98, 98, 60, 99, 99, 1, 84, 17];//, 98, 60];

        println!("AVL Tree INIT = {:?}", avl);
        for &x in vec_in.iter() {
            println!("inserting {} ...", x);
            avl.insert_node(x);
            // println!("size is now = {}", avl.size());
            avl.print();
            println!("height = {}", avl.height());
            println!("leaves = {}", avl.count_leaves());
            println!();
        }
    
        println!("inserting {} ...", 98);
        avl.insert_node(98);
        // println!("size is now = {}", avl.size());
        avl.print();
        println!("height = {}", avl.height());
        println!("leaves = {}", avl.count_leaves());
        println!();


    
        // vec_in = vec![17, 84, 99, 5, 1, 60];
        // for &x in vec_in.iter() {
        //     println!("deleting {} ...", x);
        //     avl.delete_node(x);
        //     println!("size is now = {}", avl.size());
        //     avl.print();
        //     println!("height = {}", avl.height());
        //     println!("leaves = {}", avl.count_leaves());
        //     println!();
        // }
 }
 pub fn delete_avl() {
    let mut avl: avl_tree::AVLTree<u32> = avl_tree::AVLTree::new();
    
    for x in 1..=10 {
        avl.insert_node(x);
        println!("inserting ... {}", x);
    }
    avl.print();
    println!("height = {}", avl.height());
    println!("leaves = {}", avl.count_leaves());
    println!("==========================");
    println!("deleting {} ...", 4);
    avl.delete_node(4);
    println!("height = {}", avl.height());
    println!("leaves = {}", avl.count_leaves());
    avl.print();
    println!("==========================");
    println!("deleting {} ...", 5);
    avl.delete_node(5);
    println!("height = {}", avl.height());
    println!("leaves = {}", avl.count_leaves());
    avl.print();
    println!("==========================");
    println!("deleting {} ...", 8);
    avl.delete_node(8);
    println!("height = {}", avl.height());
    println!("leaves = {}", avl.count_leaves());
    avl.print();
    println!("==========================");
    println!("deleting {} ...", 7);
    avl.delete_node(7);
    println!("height = {}", avl.height());
    println!("leaves = {}", avl.count_leaves());
    avl.print();
    println!("==========================");
    println!("deleting {} ...", 3);
    avl.delete_node(3);
    println!("height = {}", avl.height());
    println!("leaves = {}", avl.count_leaves());
    avl.print();
    println!("==========================");
    println!("deleting {} ...", 2);
    avl.delete_node(2);
    println!("height = {}", avl.height());
    println!("leaves = {}", avl.count_leaves());
    avl.print();
    println!("==========================");
    println!("deleting {} ...", 1);
    avl.delete_node(1);
    println!("height = {}", avl.height());
    println!("leaves = {}", avl.count_leaves());
    avl.print();
    println!("==========================");
    println!("deleting {} ...", 9);
    avl.delete_node(9);
    println!("height = {}", avl.height());
    println!("leaves = {}", avl.count_leaves());
    avl.print();
    println!("==========================");
    println!("deleting {} ...", 6);
    avl.delete_node(6);
    println!("height = {}", avl.height());
    println!("leaves = {}", avl.count_leaves());
    avl.print();
    println!("==========================");
    println!("deleting {} ...", 10);
    avl.delete_node(10);
    println!("height = {}", avl.height());
    println!("leaves = {}", avl.count_leaves());
    avl.print();

    println!();

}
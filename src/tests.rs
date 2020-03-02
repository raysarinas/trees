use super::*;
use avl_tree::AVLTreeTraits;
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

// pub fn it_works() {
//     // Test TreeNode
//     let treenode: red_black_tree::TreeNode<u32> = red_black_tree::TreeNode::new(5);
        
//     println!("{:?}", treenode.value());
//     println!("{:?}", treenode.color());

//     treenode.set_color(red_black_tree::NodeColor::Black);
//     println!("{:?}", treenode.color());

//     treenode.set_value(10);
//     println!("{:?}", treenode.value());


//     // Test RBTree
//     println!("\n==== Start Testing RBTree Here ====\n");
//     let mut rbt: red_black_tree::RBTree<u32> = red_black_tree::RBTree::new();
//     println!("empty height = {}", rbt.height());
//     println!("no leaves = {}", rbt.count_leaves());
//     println!();

//     let mut vec_in = vec![5, 70, 35, 8, 100, 60, 120, 1, 84, 17];
//     for &x in vec_in.iter() {
//         println!("Inserting {} ...", x);
//         rbt.insert_node(x);
//         println!("size = {}", rbt.size());
//         rbt.print();
//         println!("height = {}", rbt.height());
//         println!("number of leaves = {}", rbt.count_leaves());
//         println!();
//     }
//     vec_in = vec![35, 5, 1, 8, 17, 60, 70, 84, 100, 120];
//     for &x in vec_in.iter() {
//         assert!(rbt.search(x).is_some());
//         assert!(rbt.contains(x) == true);
//     }
//     assert!(rbt.search(10).is_none());
//     assert!(rbt.contains(10) == false);

//     println!("\n==== Start Testing DELETE RBTree Here ====\n");
//     let mut rbt2: red_black_tree::RBTree<u32> = red_black_tree::RBTree::new();
//     let mut vec_in = vec![30, 20, 40, 10, 50];
//     for &x in vec_in.iter() {
//         println!("Inserting {} ...", x);
//         rbt2.insert_node(x);
//         println!("size = {}", rbt2.size());
//         rbt2.print();
//         println!("height = {}", rbt2.height());
//         println!("leaves = {}", rbt2.count_leaves());
//         println!();
//     }

//     vec_in = vec![10, 20, 30];
//     for &x in vec_in.iter() {
//         println!("Deleting {} ...", x);
//         rbt2.delete_node(x);
//         println!("size = {}", rbt2.size());
//         rbt2.print();
//         println!("height = {}", rbt2.height());
//         println!("num leaves = {}", rbt2.count_leaves());
//         println!(); 
//     }
// }

// pub fn test_delete_rbt() {
//     let mut rbt: red_black_tree::RBTree<u32> = red_black_tree::RBTree::new();

//     for x in 1..=10 {
//         rbt.insert_node(x);
//         println!("inserting ... {}", x);
//     }

//     assert!(rbt.is_empty() == false);
//     rbt.print();

//     println!("deleting everything ..");

//     let mut vec_in = vec![6, 4, 2, 8, 9, 7, 3, 1, 5, 10];
//     for &x in vec_in.iter() {
//         println!("deleting {} ...", x);
//         rbt.delete_node(x);
//     }
//     assert!(rbt.is_empty() == true);
//     rbt.print();
// }

// pub fn delete_cases_1_3_5_6() {
//     let mut rbt: red_black_tree::RBTree<u32> = red_black_tree::RBTree::new();
    
//     for x in 1..=10 {
//         rbt.insert_node(x);
//         println!("inserting ... {}", x);
//     }

//     let mut vec_in: Vec<u32> = vec![4, 6, 8, 1, 5, 9, 2];
//     for &x in vec_in.iter() {
//         println!("deleting ... {}", x);
//         rbt.delete_node(x);
//         rbt.print();
//     }
// }

// pub fn delete_cases_2_4() {
//     println!("\n==== Start Testing DELETE RBTree Here ====\n");

//     let mut rbt: red_black_tree::RBTree<u32> = red_black_tree::RBTree::new();
//     let mut vec_in: Vec<u32> = vec![5, 70, 35, 8, 98, 60, 99, 99, 1, 84, 17];

//     for &x in vec_in.iter() {
//         println!("inserting {} ...", x);
//         rbt.insert_node(x);
//         println!("size is now = {}", rbt.size());
//         rbt.print();
//         println!("height = {}", rbt.height());
//         println!("leaves = {}", rbt.count_leaves());
//         println!();
//     }

//     vec_in = vec![17, 84, 99, 5, 1, 60];
//     for &x in vec_in.iter() {
//         println!("deleting {} ...", x);
//         rbt.delete_node(x);
//         println!("size is now = {}", rbt.size());
//         rbt.print();
//         println!("height = {}", rbt.height());
//         println!("leaves = {}", rbt.count_leaves());
//         println!();
//     }
// }

// pub fn avl_test() {
//     println!("\n==== Start Testing AVLTree Here ====\n");

//     let mut avl: avl_tree::AVLTree<u32> = avl_tree::AVLTree::new();
//     let mut vec_in: Vec<u32> = vec![5, 70, 35, 8, 98, 60, 99, 99, 1, 84, 17];
//     for &x in vec_in.iter() {
//         println!("inserting {} ...", x);
//         avl.insert_node(x);
//         println!("size is now = {}", avl.size());
//         avl.print();
//         println!("height = {}", avl.height());
//         println!("leaves = {}", avl.count_leaves());
//         println!();
//     }

//     vec_in = vec![17, 84, 99, 5, 1, 60];
//     for &x in vec_in.iter() {
//         println!("deleting {} ...", x);
//         avl.delete_node(x);
//         println!("size is now = {}", avl.size());
//         avl.print();
//         println!("height = {}", avl.height());
//         println!("leaves = {}", avl.count_leaves());
//         println!();
//     }
// }

// pub fn delete_avl() {
//     let mut avl: avl_tree::AVLTree<u32> = avl_tree::AVLTree::new();
    
//     for x in 1..=10 {
//         avl.insert_node(x);
//         println!("inserting ... {}", x);
//     }
//     avl.print();
//     println!("height = {}", avl.height());
//     println!("leaves = {}", avl.count_leaves());
//     println!("==========================");

//     let vec_in = vec![4, 5, 8, 7, 3, 2, 1, 9, 6, 10];
//     for &x in vec_in.iter() {
//         println!("deleting {} ...", x);
//         avl.delete_node(x);
//         println!("height = {}", avl.height());
//         println!("leaves = {}", avl.count_leaves());
//         avl.print();
//         println!("==========================");
//     }
//     println!();
// }
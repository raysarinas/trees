mod avl_tree;
mod red_black_tree;
mod tests;
mod tree;

use red_black_tree::*;
use avl_tree::*;
use tests::*;
use crate::tree::*;

use std::io::{stdin, stdout, Write};

static MAIN_MENU: &str = "
1. Red Black Tree
2. AVL Tree
3. Run Benchmark Tests
4. Exit
";

static BENCHMARK_MENU: &str = "
BENCHMARK TESTS
1. Red Black Tree Tests
2. AVL Tree Tests
3. Return to main menu
";

static RBT_MENU: &str = "
RED-BLACK TREE
1. Insert a node to the red-black tree
2. Delete a node from the red-black tree
3. Count the number of leaves in the tree
4. Return the height of the tree
5. Print in-order traversal of the tree
6. Check if tree is empty
7. Return to main menu
";

static AVL_MENU: &str = "
AVL TREE
1. Insert a node to the AVL tree
2. Delete a node from the AVL tree
3. Count the number of leaves in the tree
4. Return the height of the tree
5. Print in-order traversal of the tree
6. Check if tree is empty
7. Return to main menu
";

fn get_input(prompt: &str) -> u32 {
    loop {
        print!("{} ", prompt);
        stdout().flush().unwrap();
        let mut value = String::new();
        stdin().read_line(&mut value).expect("Failed to read line");

        match value.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input! Please enter a numerical value.");
                continue;
            }
        }
    }
}

fn avl() {
    let mut avl: AVLTree<u32> = AVLTree::new();
    loop {
        println!("{}", AVL_MENU);
        let choice = get_input(">");
        match choice {
            1 => {
                let value = get_input("Value to insert:");
                let old_size = avl.size();
                avl.insert_node(value);
                if old_size < avl.size() {
                    println!("Successfully inserted {}", value);
                } else {
                    println!("Error inserting value");
                }
            },
            2 => {
                let value = get_input("Value to delete:");
                let old_size = avl.size();
                avl.delete_node(value);
                if old_size > avl.size() {
                    println!("Successfully deleted {}", value);
                } else {
                    println!("Error deleting value");
                }
            },
            3 => println!("Number of leaves: {}", avl.count_leaves()),
            4 => println!("Height: {}", avl.height()),
            5 => avl.print(),
            6 => {
                if avl.is_empty() {
                    println!("Tree is empty.");
                } else {
                    println!("Nah tree is full of leaves");
                }
            },
            7 => break,
            _ => println!("Invalid input!")
        }
    }
}

fn rbt() {
    let mut rbt: RBTree<u32> = RBTree::new();
    loop {
        println!("{}", RBT_MENU);
        let choice = get_input(">");
        match choice {
            1 => {
                let value = get_input("Value to insert:");
                let old_size = rbt.size();
                rbt.insert_node(value);
                if old_size < rbt.size() {
                    println!("Successfully inserted {}", value);
                } else {
                    println!("Error inserting value");
                }
            },
            2 => {
                let value = get_input("Value to delete:");
                let old_size = rbt.size();
                rbt.delete_node(value);
                if old_size > rbt.size() {
                    println!("Successfully deleted {}", value);
                } else {
                    println!("Error deleting value");
                }
            },
            3 => println!("Number of leaves: {}", rbt.count_leaves()),
            4 => println!("Height: {}", rbt.height()),
            5 => rbt.print(),
            6 => {
                if rbt.is_empty() {
                    println!("Tree is empty.");
                } else {
                    println!("Nah tree is full of leaves");
                }
            }
            7 => break,
            _ => println!("Invalid input!")
        }
    }
}

fn benchmark_tests() {
    loop {
        println!("{}", BENCHMARK_MENU);
        let choice = get_input(">");
        match choice {
            1 => {
                for tree_size in vec![10_000, 40_000, 70_000, 100_000, 130_000] {
                    let tree: red_black_tree::RBTree<u32> = red_black_tree::RBTree::new();
                    benchmark_insert_search(tree, tree_size);
                }
            },
            2 => {
                for tree_size in vec![10_000, 40_000, 70_000, 100_000, 130_000] {
                    let tree: avl_tree::AVLTree<u32> = avl_tree::AVLTree::new();
                    benchmark_insert_search(tree, tree_size);
                }
            },
            3 => break,
            _ => println!("Invalid input!")
        }
    }
}

fn main() {
    loop {
        println!("{}", MAIN_MENU);
        let choice = get_input(">");
        match choice {
            1 => rbt(),
            2 => avl(),
            3 => benchmark_tests(),
            4 => {
                println!("ok bye");
                break;
            },
            _ => println!("Invalid input!")
        }
    }
}

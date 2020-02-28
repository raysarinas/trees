mod avl_tree;
mod red_black_tree;
mod tests;

use red_black_tree::*;
use tests::*;

use std::io::{stdin, stdout, Write};

static MAIN_MENU: &str = "
1. Red Black Tree
2. AVL Tree
3. Run benchmark tests
4. Exit
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
            Err(_) => continue
        }
    }
}

fn avl() {
    loop {
        println!("{}", AVL_MENU);
        let choice = get_input(">");
        match choice {
            1 => {},
            2 => {},
            3 => {},
            4 => {},
            5 => {},
            6 => {},
            7 => break,
            _ => println!("Invalid input!")
        }
    }
}

fn rbt() {
    // not sure what to use for type
    let mut rbt: RBTree<u32> = RBTree::new();
    loop {
        println!("{}", RBT_MENU);
        let choice = get_input(">");
        match choice {
            1 => {
                let value = get_input("Value to insert:");
                rbt.insert_node(value); // TODO: error handling?
                println!("Inserted {}", value);
            },
            2 => {
                let value = get_input("Value to delete:");
                rbt.delete_node(value); // TODO: error handling?
                println!("Deleted {}", value);
            },
            3 => println!("{}", rbt.size()),
            4 => println!("{}", rbt.height()),
            5 => rbt.print(),
            6 => {
                if rbt.is_empty() {
                    println!("Tree is empty.");
                } else {
                    println!("nah tree is full of leaves");
                }
            }
            7 => break,
            _ => println!("Invalid input!")
        }
    }
}

fn main() {
    // loop {
    //     println!("{}", MAIN_MENU);
    //     let choice = get_input(">");
    //     match choice {
    //         1 => rbt(),
    //         2 => avl(),
    //         3 => it_works(),
    //         4 => {
    //             println!("ok bye");
    //             break;
    //         },
    //         _ => println!("Invalid input!")
    //     }
    // }
    // it_works();
    // print_test();
    delete_cases_2_4();

    // avl_test();
}

mod avl_tree;
mod red_black_tree;
mod tests;

use red_black_tree::*;
use tests::*;

use std::io::{stdin, stdout, Write};

static MAIN_MENU:&str = "
1. Red Black Tree
2. AVL Tree
3. Run benchmark tests
4. Exit
";

static RBT_MENU: &str = "
1. Insert a node to the red-black tree
2. Delete a node from the red-black tree
3. Count the number of leaves in the tree
4. Return the height of the tree
5. Print in-order traversal of the tree
6. Check if tree is empty
7. Return to main menu
";

static AVL_MENU: &str = "
1. Insert a node to the AVL tree
2. Delete a node from the AVL tree
3. Count the number of leaves in the tree
4. Return the height of the tree
5. Print in-order traversal of the tree
6. Check if tree is empty
7. Return to main menu
";

fn get_value() -> u32 {
    loop {
        print!("Value: ");
        stdout().flush().unwrap();
        let mut value = String::new();
        stdin().read_line(&mut value).expect("Failed to read line");

        match value.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => continue
        }
    }
}

fn get_choice(num_items: u32) -> u32 {
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut choice = String::new();
        stdin().read_line(&mut choice).expect("Failed to read line");

        let choice_parsed: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        if num_items < choice_parsed {
            println!("Invalid input");
        } else {
            return choice_parsed;
        }
    }
}

fn avl() {
    print!("\nAVL TREE");

    loop {
        println!("{}", AVL_MENU);
        let choice = get_choice(7);
        match choice {
            1 => {},
            2 => {},
            3 => {},
            4 => {},
            5 => {},
            6 => {},
            7 => {},
            _ => println!("Invalid input!")
        }
    }
}

fn rbt() {
    print!("\nRED-BLACK TREE");

    // not sure what to use for type
    let mut rbt: red_black_tree::RBTree<u32> = red_black_tree::RBTree::new();
    loop {
        println!("{}", RBT_MENU);
        let choice = get_choice(7);
        match choice {
            1 => {
                println!("\n[INSERT]");
                let value = get_value();
                rbt.insert_node(value); // TODO: error handling?
                println!("Inserted {}", value);
            },
            2 => {},
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
            7 => main_menu(), // would be nice to clear stack here
            _ => println!("Invalid input!")
        }
    }
}

fn main_menu() {
    println!("{}", MAIN_MENU);

    loop {
        let choice = get_choice(4);
        match choice {
            1 => rbt(),
            2 => avl(),
            3 => it_works(),
            4 => {
                println!("ok bye");
                std::process::exit(0);
            },
            _ => println!("Invalid input!")
        }
    }
}

fn main() {
    main_menu();
}

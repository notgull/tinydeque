// MIT/Apache2 License

use std::io::{self, prelude::*};
use tinydeque::ArrayDeque;

fn help() {
    println!(
        "
pb [item] - Push an item onto the back of the stack.
pf [item] - Push an item onto the back of the stack.
ob - Pop an item from the back.
of - Pop an item from the front.
c - Get capacity.
l - Get length.
e - Get whether or not it's empty.
f - Get whether or not it's full.
d - Print debug info.
h - Print this help menu.
q - Quit."
    );
}

fn main() {
    let mut test_deque: ArrayDeque<[i32; 10]> = ArrayDeque::new();

    help();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.is_empty() {
            continue;
        }
        let command = (
            line.remove(0),
            if !line.is_empty() {
                Some(line.remove(0))
            } else {
                None
            },
        );
        if line.chars().next() == Some(' ') {
            line.remove(0);
            line.pop();
        }
        let item = if line.len() > 0 {
            match line.parse::<i32>() {
                Ok(s) => s,
                Err(_) => 0,
            }
        } else {
            0i32
        };

        match command {
            ('p', Some('b')) => {
                if let Err(reject) = test_deque.try_push_back(item) {
                    println!("Unable to push element onto deque back: {}", reject);
                }
            }
            ('p', Some('f')) => {
                if let Err(reject) = test_deque.try_push_front(item) {
                    println!("Unable to push element onto deque front: {}", reject);
                }
            }
            ('o', Some('b')) => println!("{:?}", test_deque.pop_back()),
            ('o', Some('f')) => println!("{:?}", test_deque.pop_front()),
            ('c', _) => println!("Capacity: {}", ArrayDeque::<[i32; 10]>::capacity()),
            ('l', _) => println!("Length: {}", test_deque.len()),
            ('e', _) => println!(
                "Deque is{} empty",
                if test_deque.is_empty() { "" } else { " not" }
            ),
            ('f', _) => println!(
                "Deque is{} full",
                if test_deque.is_full() { "" } else { " not " }
            ),
            ('d', _) => println!("{:?}", &test_deque),
            ('h', _) => help(),
            ('q', _) => return,
            _ => println!("Unrecognized command"),
        }
    }
}

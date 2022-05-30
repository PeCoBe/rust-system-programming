/// Author PeCoBe
/// Description: Simple c example, based on C gets na fgets.

use std::io;

fn main() {
    println!("Enter a sentence");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    println!("You entered {}", input);
}

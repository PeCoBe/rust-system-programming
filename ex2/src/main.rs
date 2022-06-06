/// Author PeCoBe
/// Description: Exercises with Strings using Rust

use std::char;

fn main() {
    println!("Review on String (char array, string, and string functions)");

    //Initialize character array
    let mut array: [char;10] = ['a';10];

    array[0] = 'a';
    array[1] = 'b';
    array[2] = 'c'; 
    array[3] = 'd'; 
    array[4] = 'e'; 
    array[5] = 'f';
    array[6] = 'g';
    array[7] = 'h';
    array[8] = 'i';
    array[9] = 'j';

    //Print each character
    println!("Original char array");
    for x in array {
        print!("{}", x);
    }
    println!();

    println!("ASCII code of each char");
    for x in array {
        println!("{} - {}", x, x as u32);
    }

    // Begin Homework
    
    println!("Declare a character array with 128 rooms. Store 0 to 127 in this array and print
the corresponding character for each ascii code in the array.");
    
    let mut array_of_char : [u32; 128] = [0; 128]; 
    
    for i in 0..128 {
        array_of_char[i] = i as u32;
        println!("u32: {} - char: {:?}", array_of_char[i], char::from_u32(array_of_char[i]));
    }

    println!("In C, a string is a character array ending with null. 
In Rust string are not null terminated by default.
All strings in rust are UTF-8 encoded");

    println!("Char array");
    let a: [char; 5] = ['a', 'p', 'p', 'l', 'e'];
    println!("{:?}", a);

    println!("Create a new string");
    let mut _s = String::new(); //This is an empty string, we can load data.
    
    let data = "Initial contents";

    let mut s = data.to_string();
    println!("{}", s);
    s = String::from("newer contents");
    println!("{}", s);    

    s.push_str(" plus dlc");
    println!("After using push_str: {}", s);

    s.push('ñ');
    println!("Adding one char using push: {}", s);

    let s1 = String::from(" & knuckles");
    let s3 = s + &s1; // this is the end of s
    // Difference between String, &str, &String

    //If need to concatenate multiple strings, prefer format!
    let s10 = String::from("piotor");
    let s20 = String::from("pooper");
    let s30 = String::from("panini");

    let long_s 
}

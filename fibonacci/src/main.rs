// A program to generate nth fibonacci number
use std::io;
fn main() {
    loop {
    println!("Enter the nth number:");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u64 = choice.trim().parse().expect("Failed to read, try again");
    let mut n1 = 0;
    let mut n2 = 1;
    let mut count = 0;
    while count < choice {
        let nth = n1+n2;
        n1 = n2;
        n2 = nth;
        count = count+1;
    }
    println!("The nth fibonacci number = {n1}");
}
}

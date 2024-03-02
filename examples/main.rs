extern crate pwin

use pwin::pw;

fn main() {
    println!("Enter Username:");
    io::stdin().read_line(&mut username).expect("Failed to read username.");

    println!("Enter Password:");
    let password = pw::readpw();
    
    println!("{}, {}", username, password)
}

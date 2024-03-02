This is a simple library to get a password without the text echoing on the terminal. It only has one function and t's made to allow beginner developers to prompt users for passwords relatively easily.

Importing:

    use pwin::pw;


Usage:

    fn main() {
        print!("Enter Password:")
        let passwd = pw::readpw();
    }


Simple. Takes no arguments and returns a non-mutable String that you can cast or use however you need to.

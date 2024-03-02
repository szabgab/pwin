/// Usage: use crate::readpw

/// print!("Enter Password:");
/// let passwd = readpw();

/// returns a non mutable String

extern crate termion;

use termion::input::TermRead;
use std::io;

fn strip_nl(s: &mut String) -> String {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    let out: String = s.to_string();
    out
}

pub mod pw {
    pub fn readpw() -> String {
        let stdin = io::stdin();
        let mut stdin = stdin;
        let stdout = io::stdout();
        let mut stdout = stdout;

        let passwd = TermRead::read_passwd(&mut stdin, &mut stdout);
        println!("");
        let Ok(Some(mut password)) = passwd else { todo!() };
        let pw = strip_nl(&mut password);
        pw
    }
}

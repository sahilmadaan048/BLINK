use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read};

fn main() {
    enable_raw_mode().unwrap();

    for byte in io::stdin().bytes() {
        let b = byte.unwrap(); // Extract u8 byte
        let c = b as char; // Convert to char

        if c.is_control() {
            println!("Binary: {0:08b} ASCII: {0:3} \r", b);
        } else {
            println!("Binary: {0:08b} ASCII: {0:3} Character: {1:#?}\r", b, c);
        }
        println!("{}", c);
        if c == 'q' {
            disable_raw_mode().unwrap();
            break;
        }
    }
}

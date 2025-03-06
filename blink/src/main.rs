use std::io;
use std::io::Read;
// use std::io::{self, Read};

fn main() {
    //returns an iterator over the bytes of input from the user
    for b in io::stdin().bytes() {
        //use u8 to char
        let c = b.unwrap() as char;
        if c == 'q' {
            break;
        }
        println!("{}", c);
    }
}

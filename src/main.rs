use std::fs::File;
use std::io::{Read, BufReader};

// use std::str::Bytes;

fn main() {
    let f = File::open("_.iqs").unwrap();
    let mut bytes = BufReader::new(f).bytes();
    while let Some(Ok(b)) = bytes.next() {
        print_byte(b);
    }

    let s = "t1q0c34x7opyjrfp1q";
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        print_byte(b);
    }
}

fn print_byte(b: u8) {
    print!("{:?}", b);
}

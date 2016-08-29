extern crate byteorder;

use std::io;
use std::io::Write;
// use std::cmp::Ordering;
use std::collections::HashMap;

// use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use std::option::Option;

// 16 bytes sequence
struct Briq {
    l: [u8; 8],
    h: [u8; 8],
}

impl Briq {
    fn new() -> Briq {
        Briq { l: [0; 8], h: [0; 8] }
    }

    fn hex_l(&self) -> String {
        let mut s = "0x".to_string();
        for i in self.l.iter() {
            s.push_str(format!("{:x}", i).as_str());
        }
        s.to_string()
    }

    fn hex_h(&self) -> String {
        let mut s = "0x".to_string();
        for i in self.h.iter() {
            s.push_str(format!("{:x}", i).as_str());
        }
        s.to_string()
    }

    fn set_l(&mut self, n: u64) {
        (&mut(self.l)[..]).write_u64::<BigEndian>(n).unwrap();
    }

    fn get_l(&self) -> u64 {
        (&(self.l)[..]).read_u64::<BigEndian>().unwrap()
    }

    fn set_h(&mut self, n: u64) {
        (&mut(self.h)[..]).write_u64::<BigEndian>(n).unwrap();
    }

    fn get_h(&self) -> u64 {
        (&(self.h)[..]).read_u64::<BigEndian>().unwrap()
    }
}

impl std::fmt::Display for Briq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", "po", "pa")
    }
}

impl std::fmt::Debug for Briq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.hex_l(), self.hex_h())
    }
}

fn show(m: &HashMap<i16, Vec<Briq>>) {
    for (i, v) in m {
        println!("{}{:?}", i, v);
    }
    // println!("{}", m);
}

fn show_with_big_endian(i: i64) {
    println!("{}", i << 3);
}

fn main() {
    let mut m: HashMap<i16, Vec<Briq>> = HashMap::new();

    let b = Briq::new();
    m.insert(-1, vec![b]);
    /*
    match m.get_mut(&-1) {
        Some(ref mut v) => v.push(Briq::new()),
        _ => println!("can not get -1 bucket!")
    }
    */

    loop {
        print!("@|| ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("!!!! FAIL READ !!!!");

        let mut token = guess.split_whitespace();

        match token.next() {
            Some("quit") => {
                println!("bye!");
                break;
            },
            Some("mkbc") => {
                let b = Briq::new();
                m.insert(0, vec![b]);
            },
            Some("show") => {
                ; // do nothing
            }
            _ => {
                println!("you typed {}", guess);
            }
        }

        show(&m);
    }
}

#[test]
fn it_works() {
    show_with_big_endian(100);

    let mut b2 = Briq::new();
    b2.set_l(65536);
    println!("l: {:?}", b2.l);
    b2.set_h(65536 * 65536);
    println!("h: {:?}", b2.h);

    assert_eq!(b2.get_l(), 65536);
    assert_eq!(b2.get_h(), 65536 * 65536);

    let mut iter = "A few words".split_whitespace();

    assert_eq!(Some("A"), iter.next());
    assert_eq!(Some("few"), iter.next());
    assert_eq!(Some("words"), iter.next());
}

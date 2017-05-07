extern crate byteorder;

use std::io;
use std::io::Write;
use std::collections::HashMap;

use std::option::Option;

mod briq;
use briq::Briq;

fn show(m: &HashMap<i16, Vec<Briq>>) {
    for (i, v) in m {
        println!("{}{:?}", i, v);
    }
    // println!("{}", m);
}

fn show_with_big_endian(i: i64) {
    println!("{}", i << 3);
}

mod category;
use category::{Category, Bool, TRUE, FALSE};



fn parse() {
    let code = "a";
    for c in code.chars() {
        if c == " " {
            continue;
        } else if c == "(" {

        } else if c == ")" {

        } else {

        }

        println!("{}", c);
    }
}

fn main() {
    // 101 5
    // let mut integer1 = Category::<Bool>::new();
    // integer1.addObj(TRUE);
    // integer1.addObj(FALSE);
    // integer1.addObj(TRUE);

    // 110 6
    // let mut integer2 = Category::<Bool>::new();
    // integer2.addObj(TRUE);
    // integer2.addObj(TRUE);
    // integer2.addObj(FALSE);

    parse();

    let mut yards: HashMap<i16, Vec<Briq>> = HashMap::new();

    let b = Briq::new();
    yards.insert(-1, vec![b]);
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
                match token.next().unwrap().parse::<i16>() {
                    Ok(n) => {
                        yards.insert(n, vec![b]);
                    },
                    _ => {
                        println!("invalid arg!");
                    }
                }
            },
            Some("show") => {
                ; // do nothing
            }
            _ => {
                println!("you typed {}", guess);
            }
        }

        show(&yards);
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

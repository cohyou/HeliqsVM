use std::io;
use std::io::Write;
use std::cmp::Ordering;
use std::collections::HashMap;

struct Briq {
    l: [i8; 8],
    h: [i8; 8],
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
    match m.get_mut(&-1) {
        Some(ref mut v) => v.push(Briq::new()),
        _ => println!("wowow!")
    }

    show_with_big_endian(100);

    loop {
        print!("@|| ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("!!!! FAIL READ !!!!");

        match guess.trim_right() {
            "quit" => {
                println!("bye!");
                break;
            },
            "show" => {
                ;// do nothing
            }
            _ => {
                println!("you typed {}", guess);
            }
        }

        show(&m);
    }
}

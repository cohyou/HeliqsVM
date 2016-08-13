use std::io;
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
}

fn show(m: &HashMap<i16, Vec<Briq>>) {
    for (i, v) in m {
        println!("{}", i);
    }
    // println!("{}", m);
}

fn main() {
    let mut m: HashMap<i16, Vec<Briq>> = HashMap::new();

    let b = Briq::new();

    m.insert(-1, vec![b]);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("!!!! FAIL READ !!!!");

        if guess == "quit\n" {
            println!("bye!");
            break;
        } else {
            println!("you typed {}", guess);
            show(&m);
        }
    }
}

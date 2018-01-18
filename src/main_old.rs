// extern crate byteorder;

// use std::io;
// use std::io::Write;
// use std::collections::HashMap;

// use std::option::Option;

// mod briq;
// use briq::Briq;

/*
fn show(m: &HashMap<i16, Vec<Briq>>) {
    for (i, v) in m {
        println!("{}{:?}", i, v);
    }
    // println!("{}", m);
}

fn show_with_big_endian(i: i64) {
    println!("{}", i << 3);
}
*/

// mod category;
// use category::{Category, Bool, TRUE, FALSE};

mod vm;

fn main() {
    let mut v = vm::VM::new();

    let i1 = v.make_dbr1(0);
    v.edit_dbr1(i1, 1);

    let i2 = v.make_lmbd(usize::max_value(), i1);
    v.edit_lmbd_p(i2, 3);
    v.edit_lmbd_q(i2, 4);

    let i3 = v.make_int1(0);
    v.edit_int1(i3, 1);

    let i4 = v.make_appl(i2, usize::max_value());
    v.edit_appl_p(i4, i2);
    v.edit_appl_q(i4, i3);

    let i5 = v.make_text("wowow");
    v.edit_text(i5, "wowowow");

    v.invalidate(i3);

    v.make_int1(100);

    v.show_target_tube();

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

    /*
    parse();

    let mut yards: HashMap<i16, Vec<Briq>> = HashMap::new();

    let b = Briq::new();
    yards.insert(-1, vec![b]);
    */

    /*
    match m.get_mut(&-1) {
        Some(ref mut v) => v.push(Briq::new()),
        _ => println!("can not get -1 bucket!")
    }
    */

    /*
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
    */
}

/*
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
*/

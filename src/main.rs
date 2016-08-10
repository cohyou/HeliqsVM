use std::io;

fn main() {
    println!("wowowow");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("!!!! FAIL READ !!!!");

    println!("you typed {}", guess);
}

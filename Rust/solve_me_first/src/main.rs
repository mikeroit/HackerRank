use std::io;

#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {
    // declare vars
    let mut num_str_1 = String::new();
    let mut num_str_2 = String::new();

    // read vars
    io::stdin().read_line(&mut num_str_1).ok().expect("read error");
    io::stdin().read_line(&mut num_str_2).ok().expect("read error");

    // parse
    let mut num_1: i32 = num_str_1.trim().parse().ok().expect("parse error");
    let mut num_2: i32 = num_str_2.trim().parse().ok().expect("parse error");

    // print sum
    println!("{}", num_1 + num_2);


}

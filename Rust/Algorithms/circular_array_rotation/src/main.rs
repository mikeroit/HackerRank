<<<<<<< HEAD
// Algorithms Warmups: circular array rotation
use std::io;

fn parse_inputs() -> Vec<i32>
{
    let myVec: Vec<i32> = vec!(1, 2, 3);
    myVec
}


fn main() {
    println!("Hello, world!");
=======
// circular array rotation 
use std::io;

fn read_array_as_ints(n: i32) -> Option<Vec<i32>>
{
    let numElements = n;
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok().expect("can not read line");
    let input: Vec<i32> = 
        line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
    match input.len()
    {
        numElements => Some(input),
        _ => None
    }
}

fn read_line_as_ints() -> Option<Vec<i32>>
{
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok().expect("can not read line");
    let input: Vec<i32> = 
        line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
    match input.len()
    {
        3 => Some(input),
        _ => None
    }
}

fn main() 
{
    // read first line (n, k, q)
    // n = num array elements
    // k = num circular shuffles
    // q = num queries
    let line_one: Vec<i32> = read_line_as_ints().unwrap();
    let (n, k, q) = (line_one[0], line_one[1], line_one[2]);

    // read array
    let array: Vec<i32> = read_array_as_ints(n).unwrap();
    for element in array
    {
        println!("{0}", element);
    }



>>>>>>> 91a7213805db97d571197921869e648dcc7efef5
}

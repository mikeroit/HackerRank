use std::io;

// make a struct to hold a score
struct score
{
    clarity: i32,
    originality: i32,
    difficulty: i32
}

impl score
{
    pub fn new(nums: &Vec<&str>) -> score
    {
        score
        {
            // TODO not implimented
            clarity: 0,
            originality: 0,
            difficulty: 0
        }
    }
}


#[allow(unused_variables)]
fn main() 
{
    // declare variables to store the string inputs
    let mut alice_score = String::new();
    let mut bob_score = String::new();


    // read the inputs
    io::stdin().read_line(&mut alice_score).ok().expect("error reading");
    io::stdin().read_line(&mut bob_score).ok().expect("error reading");

    // parse the strings to Vecs
    let mut alice_splits: Vec<&char> = Vec::new();
    let bob_splits: Vec<&str> = bob_score.split(' ').collect();

    println!("{}", alice_splits[1]);
}



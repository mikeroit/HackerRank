use std::io;
use std::io::BufRead;

// solution for hackerland

// struct to hold input
struct HackerlandInput
{
    my_n: i32, // number of houses in Hackerland
    my_k: i32, // radius of single transmitter
    my_houses: Vec<i32>, // vector of house indicies
}

impl HackerlandInput
{
    fn new(n: i32, k: i32, source: Vec<i32>) -> HackerlandInput
    {
        HackerlandInput
        {
            my_n: n,
            my_k: k,
            my_houses: source,
        }
    }
}

// struct for 1-dim location in hackerland
struct Cell
{
    is_covered: bool
}


// function to parse input
fn parse_input() -> HackerlandInput
{

    let reader = io::stdin();
    let line_a: Vec<i32> = 
                        reader.lock()                     
                        .lines().next().unwrap().unwrap() 
                        .split(' ').map(|s| s.trim())     
                        .filter(|s| !s.is_empty())       
                        .map(|s| s.parse().unwrap())      
                        .collect();                     

    let line_b: Vec<i32> = 
                        reader.lock()                     
                        .lines().next().unwrap().unwrap() 
                        .split(' ').map(|s| s.trim())     
                        .filter(|s| !s.is_empty())       
                        .map(|s| s.parse().unwrap())      
                        .collect();                     

    HackerlandInput
    {
        my_n: line_a[0],
        my_k: line_a[1],
        my_houses: line_b.clone(),
    }
}

// function to check if a given number of transmitters can cover hackerland
// returns true if there exists a configuration of transmitters that satisfies all houses
fn  arrange_transmitters(hackerland: &HackerlandInput, num_transmitters: i32) -> bool
{
    // grab transmitter radius
    let radius: i32 = hackerland.my_k;

    // grab necessary hackerland size
    let max: i32 = hackerland.my_houses.iter().max().unwrap().clone();

    // make a 1-d neighborhood using max size
    let mut temp_hackerland: Vec<Cell> = Vec::new();

    for i in 0..max
    {
        temp_hackerland.push(Cell{is_covered: false});
    }

    true

}

// main
fn main() 
{
    println!("Hello, world!");
    let input = parse_input();

    // we know the trivial solution will be `n` transmitters
    let mut res = input.my_n;

}

// unit testing
#[test]
fn test_new_hackerland_input()
{
    let temp = HackerlandInput::new(5, 10, vec!(2, 4, 6));

    assert_eq!(temp.my_houses[0], 2);
    assert_eq!(temp.my_houses[1], 4);
    assert_eq!(temp.my_houses[2], 6);

    assert_eq!(temp.my_n, 5);
    assert_eq!(temp.my_k, 10);
}

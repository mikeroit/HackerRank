use std::io;
use std::str;

<<<<<<< HEAD
// make a struct to hold a score
=======
// score struct
// struct to hold score
>>>>>>> 91a7213805db97d571197921869e648dcc7efef5
#[allow(dead_code)]
struct Score
{
    name: String,
    clarity: i32,
    originality: i32,
    difficulty: i32
}

<<<<<<< HEAD
#[allow(dead_code, unused_variables)]
impl Score
{
    pub fn new(nums: &Vec<&str>) -> Score
    {
        Score
        {
            // TODO not implimented
            clarity: 0,
            originality: 0,
            difficulty: 0
        }
    }
}
#[allow(unused_variables, unused_mut)]
=======
impl Score 
{
    pub fn print(&self)
    {
        println!("Name: {0},  Score:\n{1} {2} {3}",
                 self.name, self.clarity, self.originality, self.difficulty);
    }
}

#[allow(unused_variables)]
>>>>>>> 91a7213805db97d571197921869e648dcc7efef5
fn main() 
{
    // read inputs to `&str` vars
    // declare variables to store the string inputs
    let mut alice_score = String::new();
    let mut bob_score = String::new();

    // read the inputs
    io::stdin().read_line(&mut alice_score).ok().expect("error reading");
    io::stdin().read_line(&mut bob_score).ok().expect("error reading");

    // parse the strings to Vecs
    let alice_splits: Vec<&str> = alice_score.split_whitespace().collect();
    let bob_splits: Vec<&str> = bob_score.split_whitespace().collect();

    // convert inputs to ints, store in structs
    let bob_struct: Score = Score{ 
        name: String::from("Bob"),
        clarity: str::FromStr::from_str(bob_splits[0]).unwrap(),
        originality: str::FromStr::from_str(bob_splits[1]).unwrap(),
        difficulty: str::FromStr::from_str(bob_splits[2]).unwrap()
    };

    let alice_struct: Score = Score{ 
        name: String::from("Alice"),
        clarity: str::FromStr::from_str(alice_splits[0]).unwrap(),
        originality: str::FromStr::from_str(alice_splits[1]).unwrap(),
        difficulty: str::FromStr::from_str(alice_splits[2]).unwrap()
    };

    let mut bob_total = 0;
    let mut alice_total = 0;

    if(bob_struct.clarity > alice_struct.clarity)
    {
        bob_total = bob_total + 1;
    }
    else if(alice_struct.clarity > bob_struct.clarity)
    {
        alice_total = alice_total + 1;
    }

    if(bob_struct.originality > alice_struct.originality)
    {
        bob_total = bob_total + 1;
    }
    else if(alice_struct.originality > bob_struct.originality)
    {
        alice_total = alice_total + 1;
    }

    if(bob_struct.difficulty > alice_struct.difficulty)
    {
        bob_total = bob_total + 1;
    }
    else if(alice_struct.difficulty > bob_struct.difficulty)
    {
        alice_total = alice_total + 1;
    }

    println!("{0}{1}", alice_total, bob_total);

}

// testing
// structs
#[allow(unused_variables)]
#[test]
fn test_objects()
{
    // setup test
    let bob_splits: Vec<&str> = vec!("1", "2", "3");
    let alice_splits: Vec<&str> = vec!("4", "5", "6");
    
    // convert inputs to ints
    let bob_struct: Score = Score{ 
        name: String::from("Bob"),
        clarity: bob_splits[0].parse().ok().expect("wanted a num"),
        originality: bob_splits[0].parse().ok().expect("wanted a num"),
        difficulty: bob_splits[0].parse().ok().expect("wanted a num")
    };
}






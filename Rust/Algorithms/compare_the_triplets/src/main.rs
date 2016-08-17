use std::io;

// score struct
// struct to hold score
#[allow(dead_code)]
struct Score
{
    name: String,
    clarity: i32,
    originality: i32,
    difficulty: i32
}

impl Score 
{
    pub fn print(&self)
    {
        println!("Name: {0},  Score:\n{1} {2} {3}",
                 self.name, self.clarity, self.originality, self.difficulty);
    }
}

// constructor for score struct
//#[allow(dead_code)]
//impl Score
//{
    //#[allow(unused_variables)]
    //pub fn new(nums: &Vec<&str>) -> Score
    //{
        //Score
        //{
            //// TODO not implimented
            //clarity: nums[0].parse().unwrap(),
            //originality: nums[1].parse().unwrap(),
            //difficulty: nums[2].parse().unwrap(),
        //}
    //}
//}


#[allow(unused_variables)]
fn main() 
{
    test_objects();
}

// testing
// structs
fn test_objects()
{
    // ZONE: read inputs to `&str` vars
    // declare variables to store the string inputs
    let mut alice_score = String::new();
    let mut bob_score = String::new();


    // read the inputs
    io::stdin().read_line(&mut alice_score).ok().expect("error reading");
    io::stdin().read_line(&mut bob_score).ok().expect("error reading");

    // parse the strings to Vecs
    let alice_splits: Vec<&str> = alice_score.split(' ').collect();
    let bob_splits: Vec<&str> = bob_score.split(' ').collect();
    println!("{}{}", bob_splits[0],  bob_splits[0]);

    // ZONE: convert inputs to ints
    let bob_struct: Score = Score{ 
        name: String::from("Bob"),
        clarity: bob_splits[0].parse::<i32>().unwrap(),
        originality: bob_splits[1].parse::<i32>().unwrap(), 
        difficulty: bob_splits[2].parse::<i32>().unwrap() 
    };

    bob_struct.print();
}






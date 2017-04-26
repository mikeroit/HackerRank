use std::io;
use std::str;

//struct to represent problem state
#[derive(Debug)]
struct State
{
    apple_loc: i32,
    orange_loc: i32,
    house: (i32, i32)
}

//methods for the state struct
impl State
{
    fn orange_inbounds(&self, fruit_loc: i32) -> bool
    {
        let (x1, x2) = self.house;
        (self.orange_loc + fruit_loc >= x1) && (self.orange_loc + fruit_loc <= x2)
    }

    fn apple_inbounds(&self, fruit_loc: i32) -> bool
    {
        let (x1, x2) = self.house;
        (self.apple_loc + fruit_loc >= x1) && (self.apple_loc + fruit_loc <= x2)
    }
}

fn main() 
{
    let mut line = String::new();
    let mut line2 = String::new();
    let mut line3 = String::new();
    let mut line4 = String::new();
    let mut line5 = String::new();

    // read house locations
    io::stdin().read_line(&mut line).ok().expect("error reading");
    let splits: Vec<&str> = line.split_whitespace().collect();
    let s: i32 = splits[0].parse::<i32>().unwrap();
    let t: i32 = splits[1].parse::<i32>().unwrap();

    // read tree locations
    io::stdin().read_line(&mut line2).ok().expect("error reading");
    let splits2: Vec<&str> = line2.split_whitespace().collect();
    let a: i32 = splits2[0].parse::<i32>().unwrap();
    let b: i32 = splits2[1].parse::<i32>().unwrap();

    //make a state
    let problem_state: State = State {
        apple_loc: a,
        orange_loc: b,
        house: (s, t)
    };

    //read m and n values
    io::stdin().read_line(&mut line3).ok().expect("error reading");
    let splits3: Vec<&str> = line3.split_whitespace().collect();
    let m: i32 = splits3[0].parse::<i32>().unwrap();
    let n: i32 = splits3[1].parse::<i32>().unwrap();

    //read apples and oranges
    io::stdin().read_line(&mut line4).ok().expect("error reading");
    io::stdin().read_line(&mut line5).ok().expect("error reading");

    let a_strings: Vec<&str> = line4.split_whitespace().collect();
    let o_strings: Vec<&str> = line5.split_whitespace().collect();

    let apples: Vec<i32> = a_strings.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let oranges: Vec<i32> = o_strings.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();

    //compute
    let mut num_apples = 0;
    let mut num_oranges = 0;

    for i in 0..m
    {
        if(problem_state.apple_inbounds(apples[i as usize]))
        {
            num_apples = num_apples + 1;
        }
    }

    for i in 0..n
    {
        if(problem_state.orange_inbounds(oranges[i as usize]))
        {
            num_oranges = num_oranges + 1;
        }
    }

    println!("{0}", num_apples);
    println!("{0}", num_oranges);

}


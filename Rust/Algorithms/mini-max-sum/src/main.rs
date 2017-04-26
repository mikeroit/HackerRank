use std::io;
use std::str;

fn main() 
{
    // read inputs to `&str` vars
    // declare variables to store the string inputs
    let mut input = String::new();

    // read the inputs
    io::stdin().read_line(&mut input).ok().expect("error reading");

    // parse the strings to Vecs
    let splits: Vec<&str> = input.split_whitespace().collect();
    
    let numbers: Vec<i64> = splits.into_iter().map(|x| x.parse::<i64>().unwrap()).collect();

    //declare a max and min. make them mutable
    let mut max: i64 = 0;
    let mut min: i64 = 0x7FFFFFFF;

    for i in 0..5
    {
        let mut sum: i64 = 0;
        for j in 0..5
        {
            if i != j
            {
                sum = sum + numbers[j];
            }
        }

        if sum > max
        {
            max = sum;
        }

        if sum < min
        {
            min = sum;
        }

    }

    println!("{0} {1}", min, max);
    
}


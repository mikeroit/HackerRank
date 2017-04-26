use std::io;
use std::str;

fn main() 
{
    // read inputs to `&str` vars
    // declare variables to store the string inputs
    let mut num_candles_input = String::new();
    let mut candles_input = String::new();

    // read the inputs
    io::stdin().read_line(&mut num_candles_input).ok().expect("error reading");
    io::stdin().read_line(&mut candles_input).ok().expect("error reading");

    // parse the inputs
    let num_candles_str: Vec<&str> = num_candles_input.split_whitespace().collect();
    let num_candles: i32 = num_candles_str[0].parse::<i32>().unwrap();
    
    let temp_heights: Vec<&str> = candles_input.split_whitespace().collect();
    let heights: Vec<i32> = temp_heights.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();

    //calculate
    let mut max_val: i32 = 0;
    let mut max_count: i32 = 0;

    for i in 0..num_candles
    {
        if heights[i as usize] > max_val
        {
            max_val = heights[i as usize];
            max_count = 1;
        }

        else if heights[i as usize] == max_val
        {
            max_count = max_count + 1;
        }
    }

    println!("{0}", max_count);
}


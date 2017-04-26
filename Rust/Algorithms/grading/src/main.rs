use std::io;
use std::str;

fn find_next_mult(source: i32, mult: i32) -> i32
{
    let mut temp = source;

    while(temp % mult != 0) {temp = temp + 1;}

    temp
}

fn main() 
{
    // read inputs to `&str` vars
    // declare variables to store the string inputs
    let mut input_size = String::new();
    let mut input_grades: Vec<i32> = vec!();

    // read the inputs
    io::stdin().read_line(&mut input_size).ok().expect("error reading");
    let splits: Vec<&str> = input_size.split_whitespace().collect();
    let size: i32 = splits[0].parse::<i32>().unwrap();

    for i in 0..size
    {
        let mut temp_str = String::new();
        io::stdin().read_line(&mut temp_str).ok().expect("error reading");
        let temp_splits: Vec<&str> = temp_str.split_whitespace().collect();
        let temp_grade: i32 = temp_splits[0].parse::<i32>().unwrap();

        if temp_grade >= 38
        {
            let next_mult: i32 = find_next_mult(temp_grade, 5);

            if next_mult - temp_grade < 3 {input_grades.push(next_mult)}
            else {input_grades.push(temp_grade);}

        }

        else {input_grades.push(temp_grade);}
        
    }

    for i in 0..size
    {
        println!("{0}", input_grades[i as usize]);
    }

    // parse the strings to Vecs
    //let splits: Vec<&str> = input.split_whitespace().collect();
    
    //let numbers: Vec<i64> = splits.into_iter().map(|x| x.parse::<i64>().unwrap()).collect();
    
}


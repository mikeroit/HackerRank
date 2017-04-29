use std::io;

fn crunch() -> Vec<String>
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("can not read line");
    let splits: Vec<&str> = input.split_whitespace().collect();

    splits.into_iter().map(|x| x.to_string()).collect::<Vec<String>>()
    
}


fn main() {
    let arg0: Vec<i32> = crunch().into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let n = arg0[0];

    let chocolate: Vec<i32> = crunch().into_iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let arg2: Vec<i32> = crunch().into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let d = arg2[0];
    let m = arg2[1];

    let mut num_breaks: i32 = 0;

    for i in 0..(n - m + 1)
    {
        let mut sum: i32 = 0;
        for j in 0..m
        {
            sum = sum + chocolate[(i + j) as usize];
        }

        if sum == d
        {
            num_breaks += 1;
        }
    }

    println!("{0}", num_breaks);

}


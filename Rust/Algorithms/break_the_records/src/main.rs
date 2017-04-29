use std::io;

fn crunch() -> Vec<String>
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("can not read line");
    let splits: Vec<&str> = input.split_whitespace().collect();

    splits.into_iter().map(|x| x.to_string()).collect::<Vec<String>>()
    
}


fn main() {
    //grab inputs
    let arg0: Vec<i32> = crunch().into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let games: Vec<i32> = crunch().into_iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let n: i32 = arg0[0];

    let mut max_score = games[0];
    let mut min_score = games[0];

    let mut delta_max = 0;
    let mut delta_min = 0;

    for i in 1..n
    {
        let temp_score = games[i as usize];

        if temp_score < min_score
        {
            min_score = temp_score;
            delta_min = delta_min + 1;
        }

        if temp_score > max_score
        {
            max_score = temp_score;
            delta_max = delta_max + 1;
        }
    }

    println!("{0} {1}", delta_max, delta_min);
}


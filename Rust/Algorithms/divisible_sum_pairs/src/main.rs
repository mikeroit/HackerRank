use std::io;

fn flush() -> Vec<String>
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("can not read line");
    let splits: Vec<&str> = input.split_whitespace().collect();

    splits.into_iter().map(|x| x.to_string()).collect::<Vec<String>>()
    
}


fn main() {
    //grab inputs

    let arg0: Vec<i32> = flush().into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let n = arg0[0];
    let k = arg0[1];

    let input: Vec<i32> = flush().into_iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut agg: i32 = 0;

    for i in 0..n
    {
        for j in (i + 1)..(n)
        {
            if (input[i as usize] + input[j as usize]) % k == 0
            {
                agg = agg + 1;
            }
        }
    }

    println!("{0}", agg);

}


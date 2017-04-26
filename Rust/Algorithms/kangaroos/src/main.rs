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

    let values: Vec<i32> = crunch().into_iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let c0 = values[0];
    let v0 = values[1];
    let c1 = values[2];
    let v1 = values[3];

    //compute
    if c0 == c1 && v0 == v1
    {
        println!("YES");
    }

    else if c0 <= c1 && v0 <= v1
    {
        println!("NO");
    }

    else 
    {
        if (c1 - c0) % (v0 - v1) == 0  {
            println!("YES");
        }
        else {
            println!("NO");
        }
    }

}


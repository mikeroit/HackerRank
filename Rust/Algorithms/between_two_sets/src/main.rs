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
    let size_a = arg0[0];
    let size_b = arg0[1];

    let arg1: Vec<i32> = crunch().into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let arg2: Vec<i32> = crunch().into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    println!("{:?}", arg1);
    println!("{:?}", arg2);

    //determine range as max of B
    let mut max: i32 = 0;
    for i in 0..size_b 
    {
        if arg2[i as usize] > max
        {
            max = arg2[i as usize];
        }
    }

    let mut res: i32 = 0;
    for j in 1..max 
    {
        let mut isBetween: bool = true;

        for x in 0..size_a
        {
            if j % arg1[x as usize] != 0
            {
                isBetween = false;
            }
        }

        for y in 0..size_b
        {
            if arg2[y as usize] % j != 0
            {
                isBetween = false;
            }
        }

        if isBetween
        {
            res = res + 1;
        }
        
    }

    println!("{0}", res);


}


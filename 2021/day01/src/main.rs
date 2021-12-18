fn main()
{
    // Part 1 =============================================\
    let file_name = "input.txt";
    let mut count = 0;
    let mut prev = 0;

    for line in std::fs::read_to_string(file_name).expect("file not found").lines()
    {
        let curr: i32 = line.trim().parse().unwrap();

        if prev < curr
        {
            count += 1;
        }
        prev = curr;
    }
    println!("Part 1 answer: {}", count - 1);


    // Part 2 =============================================\
    let file_name = "input.txt";
    let mut count2 = 0;
    let mut q = Vec::new();

    for line in std::fs::read_to_string(file_name).expect("file not found").lines()
    {
        let curr: i32 = line.trim().parse().unwrap();

        if q.len() < 4
        {
            q.push(curr);
        }
        else
        {
            // check if increasing
            if q[0] < q[3]
            {
                count2 += 1;
            }

            // move the window along
            q.remove(0);
            q.push(curr);
        }
    }

    // last added element did not get checked after being added
    if q[0] < q[3]
    {
        count2 += 1;
    }
    println!("Part 2 answer: {}", count2);
}
fn main() {
    let file_name = "input.txt";

    // Part 1
    {
        let mut x = 0;
        let mut y = 0;
        for line in std::fs::read_to_string(file_name).expect("file not found").lines()
        {
            let vec = line.split(' ').collect::<Vec<&str>>();
            let val: i32 = vec[1].trim().parse().unwrap();

            match vec[0].trim()
            {
                "forward" => x += val,
                "down"    => y += val,
                "up"      => y -= val,
                &_        => continue,
            }
        }
        println!("{} x {} = {}", x, y, x * y);
    }


    // Part 2
    {
        let mut x = 0;
        let mut y = 0;
        let mut aim = 0;
        for line in std::fs::read_to_string(file_name).expect("file not found").lines()
        {
            let vec = line.split(' ').collect::<Vec<&str>>();
            let val: i32 = vec[1].trim().parse().unwrap();

            match vec[0].trim()
            {
                "down"    => aim += val,
                "up"      => aim -= val,
                "forward" => {
                    x += val;
                    y += aim * val;
                },
                &_        => continue,
            }
        }
        println!("{} x {} = {}", x, y, x * y);
    }

}

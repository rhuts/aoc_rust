fn main() {

    // Part 1
    let file_name = "input.txt";
    
    // positive elements indicate '1' is most common and vice versa
    let mut v = vec![0; 12];
    let input = std::fs::read_to_string(file_name).expect("file not found");
    let data: Vec<&str> = input.lines().collect();
    let num_cols = data[0].len();

    for line in data
    {
        for i in 0..num_cols
        {
            v[i] += if &line[i..i+1] == "1" { 1 } else { -1 };
        }
    }
    
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in (0..num_cols).rev().step_by(1)
    {
        if v[num_cols - 1 - i] > 0
        {
            gamma += i32::pow(2, i as u32);
        }
        else
        {
            epsilon += i32::pow(2, i as u32);
        }
    }
    println!("{} x {} = {}", gamma, epsilon, gamma * epsilon);
    
    // Part 2
    
    // life support rating = oxygen rating * CO2 rating
    
    // find oxygen rating:
    // determine the most common value in the current bit pos (start at 0)
    // keep only numbers with that bit in that position
    // if 0 and 1 are equally common, keep values with a 1 in that position
    
    // find CO2 rating:
    // determing the least common value in the current bit position
    // keep only numbers with that bit in that position
    // if 0 and 1 are equally common, keep values with a 1 in that position
    println!("{:?}", v);
}

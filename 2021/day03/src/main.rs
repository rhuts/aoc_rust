fn get_most_common_bit<'a>(ratings: &'a mut Vec<&str>, pos: i32, is_most_common: bool) -> &'a str
{
    let mut count: i32 = 0;
    for line in ratings
    {
        count += if &line[pos as usize..pos as usize +1] == "1" { 1 } else { -1 };
    }

    if is_most_common // oxygen rating
    {
        return if count >= 0 { "1" } else { "0" };
    }
    else // CO2 rating
    {
        return if count >= 0 { "0" } else { "1" };
    }
}

fn remove_without_bit(ratings: &mut Vec<&str>, pos: i32, val: &str)
{
    for i in (0..ratings.len()).rev().step_by(1)
    {
        if i < ratings.len() && &ratings[i][pos as usize..pos as usize +1] != val
        {
            ratings.remove(i);
        }
    }
}

fn binary_diagnostic() -> (i32, isize)
{
    // Part 1
    let file_name = "input.txt";
    
    // positive elements indicate '1' is most common and vice versa
    let mut v = vec![0; 12];
    let input = std::fs::read_to_string(file_name).expect("file not found");
    let data: Vec<&str> = input.lines().collect();
    let num_cols = data[0].len();

    for line in &data // borrow 'data' to avoid moving
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
    let power_consumption = gamma * epsilon;
    
    // Part 2
    
    // life support rating = oxygen rating * CO2 rating
    
    // find oxygen rating:
    // determine the most common value in the current bit pos (start at 0)
    // keep only numbers with that bit in that position
    // if 0 and 1 are equally common, keep values with a 1 in that position
    // repeat this process until you are left with just one number -> that is the rating
    let mut o_ratings: Vec<&str> = data;
    let mut pos: i32 = 0;
    while o_ratings.len() != 1
    {
        if pos > 11 { break; }

        let mut copy: Vec<&str> = o_ratings.to_owned();

        let val: &str = get_most_common_bit(&mut copy, pos, true);
        
        remove_without_bit(&mut o_ratings, pos, val);

        pos += 1;
    }
    
    // find CO2 rating:
    // determing the least common value in the current bit position
    // keep only numbers with that bit in that position
    // if 0 and 1 are equally common, keep values with a 1 in that position
    // repeat this process until you are left with just one number -> that is the rating
    pos = 0;
    let input2 = std::fs::read_to_string(file_name).expect("file not found");
    let data2: Vec<&str> = input2.lines().collect();
    let mut co_ratings: Vec<&str> = data2;
    while co_ratings.len() != 1
    {
        if pos > 11 { break; }

        let mut copy: Vec<&str> = co_ratings.to_owned();

        let val: &str = get_most_common_bit(&mut copy, pos, false);

        remove_without_bit(&mut co_ratings, pos, val);

        pos += 1;
    }

    let oxygen: isize = isize::from_str_radix(o_ratings[0], 2).unwrap();
    let co2: isize = isize::from_str_radix(co_ratings[0], 2).unwrap();
    let life_support = oxygen * co2;
    
    return (power_consumption, life_support);
}

fn main() {
    let ans = binary_diagnostic();
    println!("Power consumption is {}", ans.0);
    println!("Life support rating is {}", ans.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_diagnostic_test() {
        assert_eq!(binary_diagnostic(), (2648450, 2845944));
    }
}

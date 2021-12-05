pub fn solve_a() -> u32 {
    let values: Vec<String> = include_str!("input/day3a.txt")
        .lines()
        .map(|string| string.parse().unwrap())
        .collect();
    let mut gamma_rate: String = 0.to_string();
    let mut epsilon_rate: String = 0.to_string();
    let mut times_zero = 0;
    let mut times_one = 0;

    for n in 0..12 {
        for value in &values {
            let chars: Vec<char> = value.chars().collect();
            let current_char = chars[n];
            if current_char == '0' {
                times_zero += 1;
            } else {
                times_one += 1;
            }
        }
        if times_one > times_zero {
            gamma_rate += "1";
            epsilon_rate += "0";
        } else {
            gamma_rate += "0";
            epsilon_rate += "1";
        }
        times_one = 0;
        times_zero = 0;
    }

    let ans_g = isize::from_str_radix(&gamma_rate.to_string(), 2).unwrap();
    let ans_e = isize::from_str_radix(&epsilon_rate.to_string(), 2).unwrap();
    return (ans_g * ans_e).try_into().unwrap();
}

pub fn solve_b() -> u32 {
    let values: Vec<String> = include_str!("input/day3a.txt")
        .lines()
        .map(|string| string.parse().unwrap())
        .collect();

    let mut oxygen_generator_rating: Vec<String> = Vec::from_iter(values.iter().cloned());
    let mut carbon_dioxide_rating: Vec<String> = Vec::from_iter(values.iter().cloned());

    let mut times_zero = 0;
    let mut times_one = 0;

    for n in 0..12 {
        for value in &oxygen_generator_rating {
            let chars: Vec<char> = value.chars().collect();
            let current_char = chars[n];
            if current_char == '0' {
                times_zero += 1;
            } else {
                times_one += 1;
            }
        }
        if times_one >= times_zero {
            let mut tem_vec: Vec<String> = Vec::new();
            for value in &oxygen_generator_rating {
                if value.chars().nth(n) == Some('1') {
                    tem_vec.push(value.to_string());
                }
            }
            oxygen_generator_rating = tem_vec;
        } else {
            let mut tem_vec: Vec<String> = Vec::new();
            for value in &oxygen_generator_rating {
                if value.chars().nth(n) == Some('0') {
                    tem_vec.push(value.to_string());
                }
            }
            oxygen_generator_rating = tem_vec;
        }
        times_one = 0;
        times_zero = 0;
    }

    for n in 0..12 {
        if &carbon_dioxide_rating.len() == &1 {
            break;
        }
        for value in &carbon_dioxide_rating {
            let chars: Vec<char> = value.chars().collect();
            let current_char = chars[n];
            if current_char == '0' {
                times_zero += 1;
            } else {
                times_one += 1;
            }
        }
        if times_one < times_zero {
            let mut tem_vec: Vec<String> = Vec::new();
            for value in &carbon_dioxide_rating {
                if value.chars().nth(n) == Some('1') {
                    tem_vec.push(value.to_string());
                }
            }
            carbon_dioxide_rating = tem_vec;
        } else {
            let mut tem_vec: Vec<String> = Vec::new();
            for value in &carbon_dioxide_rating {
                if value.chars().nth(n) == Some('0') {
                    tem_vec.push(value.to_string());
                }
            }
            carbon_dioxide_rating = tem_vec;
        }
        times_one = 0;
        times_zero = 0;
    }

    let oxy = isize::from_str_radix(&carbon_dioxide_rating[0].to_string(), 2).unwrap();
    let co2 = isize::from_str_radix(&oxygen_generator_rating[0].to_string(), 2).unwrap();

    return (oxy * co2).try_into().unwrap();
}
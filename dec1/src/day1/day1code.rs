use fancy_regex::Regex;
use std::{fs, collections::HashMap}; // Filesystem

pub fn run() {
    println!("Hello, world!");
    let mut sum = 0;

    // read input to var
    let input = fs::read_to_string("day1/input.txt").expect("Must be readable");
    let lines: Vec<String> = input.split("\r\n").map(str::to_string).collect();
    
    // find numbers in each line
    let digit = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let final_digit = Regex::new(r".*(\d|one|two|three|four|five|six|seven|eight|nine).*$").unwrap();
    
    let mut num_map: HashMap<&str, u32> = HashMap::new();
    num_map.insert("zero", 0);
    num_map.insert("one", 1);
    num_map.insert("two", 2);
    num_map.insert("three", 3);
    num_map.insert("four", 4);
    num_map.insert("five", 5);
    num_map.insert("six", 6);
    num_map.insert("seven", 7);
    num_map.insert("eight", 8);
    num_map.insert("nine", 9);

    
    for line in lines{
            let captures = digit.captures(&line).expect("Error running Regex").expect("No match");
            let captures2 = final_digit.captures(&line).expect("Error running Regex2").expect("No match2");
            
            let group = captures.get(0).expect("No group");
            let group2 = captures2.get(1).expect("No group2");
            
            let first_digit = extract_digit(group, &num_map);
            let last_digit = extract_digit(group2, &num_map);
            
            let mut result = first_digit.to_string();
            result.push_str(&last_digit.to_string());

            sum = sum + result.parse::<i32>().unwrap();        
    }

    println!("value is {}", sum);
}

fn extract_digit(group: fancy_regex::Match, num_map: &HashMap<&str, u32>) -> u32{
    let result_string = group.as_str().to_owned().parse::<u32>();
    let result:u32;
    
    if result_string.is_err() {
        result = num_map.get(group.as_str()).unwrap().to_owned();
    } else {
        result = result_string.unwrap();
    }

    return result;
}

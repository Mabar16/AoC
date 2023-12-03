use fancy_regex::Regex;
use std::fs; // Filesystem


fn main() {
    println!("Hello, world!");
    let mut sum = 0;

    // read input to var
    let input = fs::read_to_string("src/input.txt").expect("Must be readable");
    let lines: Vec<String> = input.split("\r\n").map(str::to_string).collect();

    // let first = String::from(lines.next().unwrap());
    
    // find numbers in each line
    let digit = Regex::new(r"\d").unwrap();
    let final_digit = Regex::new(r".*(\d).*$").unwrap();
    
    
    for line in lines{
            let captures = digit.captures(&line).expect("Error running Regex").expect("No match");
            let captures2 = final_digit.captures(&line).expect("Error running Regex2").expect("No match2");
            
            let group = captures.get(0).expect("No group");
            let group2 = captures2.get(1).expect("No group2");
            
            let mut result_string = group.as_str().to_owned();
            result_string.push_str(group2.as_str());

            sum = sum + result_string.parse::<i32>().unwrap();
        
    }

    println!("value is {}", sum);
}

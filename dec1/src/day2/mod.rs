use fancy_regex::Regex;
use std::{fs, collections::HashMap}; // Filesystem

pub fn run(){
    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;

    let total_input = fs::read_to_string("day2/input.txt").expect("Must be readable");
    let games: Vec<String> = total_input.split("\r\n").map(str::to_string).collect();
        
    // find numbers in each line
    let game_pattern = Regex::new(r"(?<=Game )\d{1,3}").unwrap();
    let red_pattern = Regex::new(r"(?<red_count>\d{1,3})(?= red)").unwrap();
    let blue_pattern = Regex::new(r"(?<blue_count>\d{1,3})(?= blue)").unwrap();
    let green_pattern = Regex::new(r"(?<green_count>\d{1,3})(?= green)").unwrap();
    
    let mut sum = 0;
    let mut power = 0;
    // get game id, compare red/blue/green to max for each round (split on ;)
    for game in games{
        let hands: Vec<String> = game.split(';').map(str::to_string).collect();
        let mut valid = true;

        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;

        for hand in hands{

            let red_count = red_pattern.find(&hand).unwrap();
            if red_count.is_some(){
                let red_cubes_nr = red_count.unwrap().as_str().to_owned().parse::<i32>().unwrap();
                valid = valid && (red_cubes_nr <= red_cubes);
                if red_cubes_nr > min_red{
                    min_red = red_cubes_nr;
                }
            }

            let green_count = green_pattern.find(&hand).unwrap();
            if green_count.is_some(){
                let green_cubes_nr = green_count.unwrap().as_str().to_owned().parse::<i32>().unwrap();
                valid = valid && (green_cubes_nr <= green_cubes);
                if green_cubes_nr > min_green{
                    min_green = green_cubes_nr;
                }
            }

            let blue_count = blue_pattern.find(&hand).unwrap();
            if blue_count.is_some(){
                let blue_cubes_nr = blue_count.unwrap().as_str().to_owned().parse::<i32>().unwrap();
                valid = valid && (blue_cubes_nr <= blue_cubes);
                if blue_cubes_nr > min_blue{
                    min_blue = blue_cubes_nr;
                }
            }
        }

        let game_id = game_pattern.find(&game).unwrap();
        
        if valid {
            sum += game_id.unwrap().as_str().to_owned().parse::<i32>().unwrap();  
        }

        power += min_blue * min_green * min_red;
    }
    
    println!("Sum: {}", &sum);
    println!("Power: {}", &power);

}
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
    // get game id, compare red/blue/green to max for each round (split on ;)
    for game in games{
        let hands: Vec<String> = game.split(';').map(str::to_string).collect();
        let mut valid = true;

        for hand in hands{

            let red_count = red_pattern.find(&hand).unwrap();
            if red_count.is_some(){
                let red_cubes_nr = red_count.unwrap().as_str().to_owned().parse::<i32>().unwrap();
                valid = valid && (red_cubes_nr <= red_cubes);
            }

            let green_count = green_pattern.find(&hand).unwrap();
            if green_count.is_some(){
                let green_cubes_nr = green_count.unwrap().as_str().to_owned().parse::<i32>().unwrap();
                valid = valid && (green_cubes_nr <= green_cubes);
            }

            let blue_count = blue_pattern.find(&hand).unwrap();
            if blue_count.is_some(){
                let blue_cubes_nr = blue_count.unwrap().as_str().to_owned().parse::<i32>().unwrap();
                valid = valid && (blue_cubes_nr <= blue_cubes);
            }
        }

        let game_id = game_pattern.find(&game).unwrap();
        
        if valid {
            println!("Valid: {}", &game);
            sum += game_id.unwrap().as_str().to_owned().parse::<i32>().unwrap();  
        } else {
            println!("Inalid: {}", &game);
        }
    }
    
    println!("{}", &sum);

}
use std::fs;
use std::str;

fn main() {

    let input_path = "../input.txt";
    let input = fs::read_to_string(input_path).expect("Could't read input file");
    let input: str::Split<&str> = input.split("\n");

    let rock = 1;
    let paper = 2;
    let scissors = 3;
    let loss = 0;
    let draw = 3;
    let win = 6;

    let mut score = 0;
    let mut i = 1;

    for throw in input {

        if throw == "" {
            continue;
        } else if throw == "A X" {
            score += scissors + loss;
        } else if throw == "A Y" {
            score += rock + draw;
        } else if throw == "A Z" {
            score += paper + win;

        } else if throw == "B X" {
            score += rock + loss;
        } else if throw == "B Y" {
            score += paper + draw;
        } else if throw == "B Z" {
            score += scissors + win;

        } else if throw == "C X" {
            score += paper + loss;
        } else if throw == "C Y" {
            score += scissors + draw;
        } else if throw == "C Z" {
            score += rock + win;
        } else {
            println!("Unknown play! Someone's gone rogue! {throw}");
        }
        
        println!("{i}  Throw: {throw}  New score: {score}");
        i += 1;
    }

    println!("Anticipated score: {score}");
}

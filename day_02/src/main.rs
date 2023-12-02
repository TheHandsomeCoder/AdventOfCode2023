use std::cmp::max;

fn main() {
    let input = include_str!("../input.txt");
    part_one(input);
    part_two(input);
}

fn part_one(input: &str) {
   let (red, green, blue) = (12, 13, 14); 

    let result: u32 = input.lines()
        .map(|line| Game::new(line))
        .filter(|game| game.is_valid(red, green, blue))
        .map(|g| g.id)
        .sum();
    println!("Part 1 Solution {:?}", result);
}

fn part_two(input: &str) {
     let result: u32 = input.lines()
        .map(|line| Game::new(line))
        .map(|g| g.get_power())
        .sum();
    println!("Part 2 Solution {}", result);
}

#[derive(Debug)]
pub struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    
    fn new(line: &str) -> Game {
        let mut red = 0;
        let mut green = 0;
        let mut blue  = 0;

        let id = line.split(":").collect::<Vec<_>>()[0].split(" ").collect::<Vec<_>>()[1].parse::<u32>().unwrap();
        let cube_reveals = line.split(":").collect::<Vec<_>>()[1].replace(";", ",");

        for ele in cube_reveals.trim().split(",") {
           let (value, colour) = ele.trim().split_once(" ").unwrap();
            match colour {
                "red" => red = max(red, value.parse::<u32>().unwrap()),
                "green" => green = max(green, value.parse::<u32>().unwrap()),
                "blue" => blue = max(blue, value.parse::<u32>().unwrap()),
                _ => continue
            }
        }

        return Game {
            id,
            red,
            green,
            blue
        }
    }

    fn is_valid(&self, red: u32, green: u32, blue: u32) -> bool {
        return self.red <= red && self.green <= green && self.blue <= blue;
    }

    fn get_power(&self) -> u32 {
        return self.red * self.blue * self.green;
    }
}
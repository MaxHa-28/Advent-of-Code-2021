use std::fs;

struct Position {
    depth: i32,
    forward: i32,
    aim: i32,
}

fn main() {
    let contents = fs::read_to_string("data.txt").expect("Error reading file");
    let all_commands: Vec<&str> = contents.split("\n").collect();

    let mut position_a = Position {
        depth: 0,
        forward: 0,
        aim: 0,
    };

    let mut position_b = Position {
        depth: 0,
        forward: 0,
        aim: 0,
    };

    for command in all_commands.iter() {
        let command_parts = command.split(" ").collect::<Vec<&str>>();
        let (text, value): (&str, i32) = (command_parts[0], command_parts[1].parse().unwrap());
        match text {
            "forward" => {
                position_a.forward += value;
                position_b.forward += value;
                position_b.depth += value * position_b.aim;
            }
            "up" => {
                position_a.depth -= value;
                position_b.aim -= value;
            }
            "down" => {
                position_a.depth += value;
                position_b.aim += value;
            }
            _ => println!("Can't match command: {}", text),
        }
    }

    println!("Day 2 a)");
    println!("What do you get if you multiply your final horizontal position by your final depth?");
    println!("Multiplied: {}", position_a.depth * position_a.forward);

    println!("Day 2 b)");
    println!("What do you get if you multiply your final horizontal position by your final depth?");
    println!("Multiplied: {}", position_b.depth * position_b.forward);
}

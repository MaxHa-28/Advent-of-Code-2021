use std::fs;

struct Position {
    depth: i32,
    forward: i32,
}

fn main() {
    let contents = fs::read_to_string("data.txt").expect("Error reading file");
    let all_commands: Vec<&str> = contents.split("\n").collect();

    let mut position = Position {
        depth: 0,
        forward: 0,
    };

    for command in all_commands.iter() {
        let command_parts = command.split(" ").collect::<Vec<&str>>();
        let (text, value): (&str, i32) = (command_parts[0], command_parts[1].parse().unwrap());
        match text {
            "forward" => position.forward += value,
            "up" => position.depth -= value,
            "down" => position.depth += value,
            _ => println!("Can't match command: {}", text),
        }
    }

    println!("What do you get if you multiply your final horizontal position by your final depth?");
    println!("Multiplied: {}", position.depth * position.forward)
}

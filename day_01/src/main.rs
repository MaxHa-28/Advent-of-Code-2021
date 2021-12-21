use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("Error reading file");
    let values: Vec<i32> = contents.split("\n").map(|s| s.parse().unwrap()).collect();

    println!("Day 1 a)");
    compare_measurements_and_print(&values);

    println!("Day 1 b)");
    let mut sliding_window: Vec<i32> = Vec::new();

    for (idx, val) in values.iter().enumerate() {
        if idx > 1 {
            sliding_window.push(val + values[idx - 1] + values[idx - 2])
        }
    }
    compare_measurements_and_print(&sliding_window);
}

fn compare_measurements_and_print(values: &Vec<i32>) {
    let mut increased_measurements = 0;

    for (idx, val) in values.iter().enumerate() {
        if idx > 0 && values[idx - 1] < *val {
            increased_measurements += 1
        }
    }

    println!("How many measurements are larger than the previous measurement?");
    println!("{} measurements!", increased_measurements);
}

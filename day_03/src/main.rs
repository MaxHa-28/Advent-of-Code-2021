use std::fs;

fn calc_power_consumption(bool_array: &Vec<bool>, num_size: &usize) -> i32 {
    let mut gamma = 0;
    let mut epsilon = 0;
    for (idx, val) in bool_array.iter().enumerate() {
        if *val == true {
            gamma += i32::pow(2, (*num_size - (idx + 1)) as u32)
        } else {
            epsilon += i32::pow(2, (*num_size - (idx + 1)) as u32)
        }
    }
    gamma * epsilon
}

fn main() {
    let contents = fs::read_to_string("data.txt").expect("Error reading file");
    let report: Vec<&str> = contents.split("\n").collect();

    let report_length = report.iter().count();
    let num_length = report.first().unwrap().chars().count();

    let mut bool_array = vec![];

    for n in 0..num_length {
        let count = report
            .join("")
            .chars()
            .skip(n)
            .step_by(num_length)
            .filter(|e| *e == '1')
            .count();

        if count > report_length / 2 {
            bool_array.push(true);
        } else {
            bool_array.push(false);
        }
    }

    let power_consumption = calc_power_consumption(&bool_array, &num_length);

    println!("What is the power consumption of the submarine?");
    println!("{} units power consumption", power_consumption)
}

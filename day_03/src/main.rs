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

fn calc_number_life_support(report: &Vec<&str>, num_size: &usize, look_for_common: bool) {
    let mut temp_report: Vec<&str> = report.to_vec();
    let mut most_common_value;

    for n in 0..*num_size {
        let report_count = temp_report.iter().count();

        let count = temp_report
            .iter()
            .filter(|&e| e.chars().nth(n).unwrap() == '1')
            .count();

        if count > report_count / 2 {
            most_common_value = '1';
        } else {
            most_common_value = '0';
        }

        if look_for_common {
            temp_report = temp_report
                .into_iter()
                .filter(|&e| e.chars().nth(n).unwrap() == most_common_value)
                .collect();
        } else {
            temp_report = temp_report
                .into_iter()
                .filter(|&e| e.chars().nth(n).unwrap() != most_common_value)
                .collect();
        }

        if temp_report.len() == 1 {
            break;
        }
    }

    let mut decimal = 0;
    for (idx, val) in temp_report.first().unwrap().() {
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

    println!("Day 3 a)");
    println!("What is the power consumption of the submarine?");
    println!("{} units power consumption", power_consumption);

    println!("Day 3 b)");
    calc_number_life_support(&report, &num_length, true);
    let test = calc_number_life_support(&report, &num_length, false);
    println!("{:?}", test);
}

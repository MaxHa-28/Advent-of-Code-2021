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

fn calc_oxygen(report: &Vec<&str>, num_size: &usize) -> i32 {
    let mut temp_report: Vec<&str> = report.to_vec();
    let mut most_common_value;

    for n in 0..*num_size {
        let report_count = temp_report.iter().count();

        let count = temp_report
            .iter()
            .filter(|&e| e.chars().nth(n).unwrap() == '1')
            .count();

        if count >= report_count / 2 {
            most_common_value = '1'
        } else {
            most_common_value = '0'
        }

        temp_report = temp_report
            .into_iter()
            .filter(|&e| e.chars().nth(n).unwrap() == most_common_value)
            .collect();

        if temp_report.len() == 1 {
            println!("{:?}", temp_report);
            break;
        }
    }

    let mut decimal = 0;
    for (idx, val) in temp_report.first().unwrap().chars().enumerate() {
        if val == '1' {
            decimal += i32::pow(2, (*num_size - (idx + 1)) as u32)
        }
    }
    println!("{}", decimal);
    decimal
}

fn calc_co2(report: &Vec<&str>, num_size: &usize) -> i32 {
    let mut temp_report: Vec<&str> = report.to_vec();
    let mut most_common_value;

    for n in 0..*num_size {
        let report_count = temp_report.iter().count();

        let count = temp_report
            .iter()
            .filter(|&e| e.chars().nth(n).unwrap() == '1')
            .count();

        if count >= report_count / 2 {
            most_common_value = '0'
        } else {
            most_common_value = '1'
        }

        if temp_report
            .iter()
            .filter(|&e| e.chars().nth(n).unwrap() == most_common_value)
            .count()
            == 0
        {
            continue;
        }

        temp_report = temp_report
            .into_iter()
            .filter(|&e| e.chars().nth(n).unwrap() == most_common_value)
            .collect();

        println!("{:?}", temp_report);

        if temp_report.len() == 1 {
            break;
        }
    }

    let mut decimal = 0;
    for (idx, val) in temp_report.first().unwrap().chars().enumerate() {
        if val == '1' {
            decimal += i32::pow(2, (*num_size - (idx + 1)) as u32)
        }
    }
    println!("{}", decimal);
    decimal
}

fn main() {
    let contents = fs::read_to_string("data.txt").expect("Error reading file");
    let report: Vec<&str> = contents.split("\n").collect();

    let report_length = report.iter().count();
    let num_length = report.first().unwrap().chars().count();

    let mut bool_array = vec![];
    let mut my_string = String::from("");

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
            my_string.push_str("1")
        } else {
            bool_array.push(false);
            my_string.push_str("0")
        }
    }

    let power_consumption = calc_power_consumption(&bool_array, &num_length);

    println!("Day 3 a)");
    println!("What is the power consumption of the submarine?");
    println!("{} units power consumption", power_consumption);
    println!("{}", usize::from_str_radix(&my_string, 2).unwrap());

    println!("Day 3 b)");
    let oxygen = calc_oxygen(&report, &num_length);
    let co2_rating = calc_co2(&report, &num_length);
    println!("What is the life support rating of the submarine?");
    println!("{}", oxygen * co2_rating);
}

use std::fs;

struct Rates {
    gamma: Vec<bool>,
    gamma_decimal: i32,
    epsilon: Vec<bool>,
    epsilon_decimal: i32,
}

fn convert_bool_array_to_decimal(bool_array: &Vec<bool>, num_size: &usize) -> i32 {
    let mut decimal = 0;
    for (idx, val) in bool_array.iter().enumerate() {
        if *val == true {
            decimal += i32::pow(2, (*num_size - (idx + 1)) as u32)
        }
    }
    decimal
}

fn main() {
    let contents = fs::read_to_string("data.txt").expect("Error reading file");
    let report: Vec<&str> = contents.split("\n").collect();

    let mut rates = Rates {
        gamma: vec![],
        gamma_decimal: 0,
        epsilon: vec![],
        epsilon_decimal: 0,
    };

    let report_length = report.iter().count();
    let num_length = report.first().unwrap().chars().count();

    for n in 0..num_length {
        let count = report
            .join("")
            .chars()
            .skip(n)
            .step_by(num_length)
            .filter(|e| *e == '1')
            .count();

        if count > report_length / 2 {
            rates.gamma.push(true);
            rates.epsilon.push(false);
        } else {
            rates.gamma.push(false);
            rates.epsilon.push(true);
        }
    }

    rates.gamma_decimal = convert_bool_array_to_decimal(&rates.gamma, &num_length);
    rates.epsilon_decimal = convert_bool_array_to_decimal(&rates.epsilon, &num_length);

    println!("What is the power consumption of the submarine?");
    println!(
        "{} units power consumption",
        rates.epsilon_decimal * rates.gamma_decimal
    )
}

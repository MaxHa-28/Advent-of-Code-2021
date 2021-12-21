use std::fs;

struct Rates {
    gamma: Vec<bool>,
    epsilon: Vec<bool>,
}

fn main() {
    let contents = fs::read_to_string("data.txt").expect("Error reading file");
    let report: Vec<&str> = contents.split("\n").collect();

    let mut rates = Rates {
        gamma: vec![],
        epsilon: vec![],
    };

    let report_length = report.iter().count();
    let num_length = report.first().unwrap().chars().count();

    let bit_array = report.join("");

    for n in 0..num_length {
        let count = bit_array
            .chars()
            .skip(n)
            .step_by(num_length)
            .filter(|e| *e == '1')
            .count();

        println!("{} {}", count, report_length);

        if count > report_length / 2 {
            rates.gamma.push(true);
            rates.epsilon.push(false);
        } else {
            rates.gamma.push(false);
            rates.epsilon.push(true);
        }
    }

    println!("{:?}", rates.gamma)
}

use std::io;

fn generate_gray_code(n: usize) -> Vec<String> {
    if n == 0 {
        return vec!["".to_string()];  // Base case: Gray code for 0 bits is an empty string
    }

    let smaller_gray_codes = generate_gray_code(n - 1);
    let mut result = Vec::new();

    // First half: prefix '0' to each smaller code
    for code in &smaller_gray_codes {
        result.push(format!("0{}", code));
    }

    // Second half: prefix '1' to each reversed smaller code
    for code in smaller_gray_codes.iter().rev() {
        result.push(format!("1{}", code));
    }

    result
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid number");

    // Generate Gray code sequence
    let gray_codes = generate_gray_code(n);

    // Print each code
    for code in gray_codes {
        println!("{}", code);
    }
}

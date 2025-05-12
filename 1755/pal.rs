use std::io::{self};

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap(); // Read only one line of input
    let s = s.trim(); // Remove any extra spaces or newline at the ends

    let mut freq = [0usize; 26];
    for ch in s.chars() {
        freq[(ch as u8 - b'A') as usize] += 1;
    }

    // Count how many characters have odd frequency
    let odd_count = freq.iter().filter(|&&f| f % 2 != 0).count();

    // Palindrome is impossible if more than 1 character has odd count
    if odd_count > 1 {
        println!("NO SOLUTION");
        return;
    }

    let mut left = String::with_capacity(s.len() / 2);
    let mut middle = String::new();

    // Construct the palindrome parts
    for i in 0..26 {
        let ch = (b'A' + i as u8) as char;
        let f = freq[i];

        if f % 2 == 0 {
            left.push_str(&ch.to_string().repeat(f / 2));
        } else {
            left.push_str(&ch.to_string().repeat(f / 2));
            middle.push_str(&ch.to_string().repeat(f % 2)); // Only one odd character will go in the middle
        }
    }

    let right: String = left.chars().rev().collect();
    println!("{}{}{}", left, middle, right);
}

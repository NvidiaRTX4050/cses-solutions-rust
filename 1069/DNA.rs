use std::io;

fn longest_repetition(dna_sequence: &str) -> usize {
    let mut max_repetition = 0;
    let mut current_repetition = 1;

    // Iterate through the sequence starting from the second character
    for i in 1..dna_sequence.len() {
        if dna_sequence[i..i+1] == dna_sequence[i-1..i] {
            current_repetition += 1;
        } else {
            max_repetition = max_repetition.max(current_repetition);
            current_repetition = 1;
        }
    }

    max_repetition.max(current_repetition)
}

fn main() {
    // Reading input
    let mut dna_sequence = String::new();
    io::stdin().read_line(&mut dna_sequence).expect("Failed to read line");

    let dna_sequence = dna_sequence.trim(); // Remove any extra whitespace

    // Handle edge case where dna_sequence is empty
    if dna_sequence.is_empty() {
        println!("0");
        return;
    }

    // Output the result
    println!("{}", longest_repetition(dna_sequence));
}

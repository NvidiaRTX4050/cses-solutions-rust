use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    
    let total = n * (n + 1) / 2;
    if total % 2 != 0 {
        println!("NO");
        return;
    }

    let mut set1 = Vec::new();
    let mut set2 = Vec::new();
    let mut target = total / 2;

    for i in (1..=n).rev() {
        if i <= target {
            set1.push(i);
            target -= i;
        } else {
            set2.push(i);
        }
    }

    println!("YES");
    println!("{}", set1.len());
    println!("{}", set1.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    println!("{}", set2.len());
    println!("{}", set2.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}

fn main() {
    let mut n: u64 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().unwrap()
    };

    let mut out = String::new();
    while n != 1 {
        out.push_str(&format!("{} ", n));
        n = if n & 1 == 0 { n >> 1 } else { n * 3 + 1 };
    }
    out.push_str("1");
    println!("{}", out);
}

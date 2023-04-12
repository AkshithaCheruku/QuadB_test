fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let s = "hello world";
    let reversed_s = reverse_string(s);
    println!("{} reversed is: {}", s, reversed_s);
}

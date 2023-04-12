fn is_palindrome(s: &str) -> bool {
    let len = s.len();
    for i in 0..len/2 {
        if s[i..=i] != s[len-1-i..=len-1-i] {
            return false;
        }
    }
    true
}

fn main() {
    let s1 = "racecar";
    let s2 = "hello";
    println!("{} is palindrome: {}", s1, is_palindrome(s1));
    println!("{} is palindrome: {}", s2, is_palindrome(s2));
}

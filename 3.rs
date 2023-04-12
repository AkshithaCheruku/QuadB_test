fn shortest_word(s: &str) -> &str {
    let mut shortest = "";
    let mut shortest_len = s.len() + 1; // initialize to a large value

    for word in s.split_whitespace() {
        let len = word.len();
        if len < shortest_len {
            shortest = word;
            shortest_len = len;
        }
    }

    shortest
}

fn main() {
    let s = "the quick brown fox jumps over the lazy dog";
    println!("The shortest word in \"{}\" is: {}", s, shortest_word(s));
}

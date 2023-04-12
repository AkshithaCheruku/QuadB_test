fn longest_common_prefix(strs: &[&str]) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let first = strs[0];
    let mut prefix = String::new();

    for (i, ch) in first.chars().enumerate() {
        for s in &strs[1..] {
            if i >= s.len() || s.chars().nth(i) != Some(ch) {
                return prefix;
            }
        }
        prefix.push(ch);
    }

    prefix
}

fn main() {
    let strs1 = vec!["flower", "flow", "flight"];
    let strs2 = vec!["dog", "racecar", "car"];
    let strs3 = vec!["abcd", "abc", "ab"];
    println!("Longest common prefix of {:?}: {}", strs1, longest_common_prefix(&strs1));
    println!("Longest common prefix of {:?}: {}", strs2, longest_common_prefix(&strs2));
    println!("Longest common prefix of {:?}: {}", strs3, longest_common_prefix(&strs3));
}

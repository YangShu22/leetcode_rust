fn is_anagram(s: String, t: String) -> bool {
    let mut s_char = vec![0; 26];
    let mut t_char = vec![0; 26];
    for c in s.chars() {
        s_char[c as usize - 'a' as usize] += 1;
    }
    for c in t.chars() {
        t_char[c as usize - 'a' as usize] += 1;
    }
    s_char == t_char
}
fn main() {
    let s = "anagram".to_string();
    let t = "nagaram".to_string();
    let res = is_anagram(s, t);
    println!("{}", res);
}

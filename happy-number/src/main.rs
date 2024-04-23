fn is_happy(n: i32) -> bool {
    let mut set = std::collections::HashSet::<i32>::new();
    // let mut string = String::new();
    let mut next =n;
    while next!=1 && !set.contains(&next) { 
        set.insert(next);
        let string = next.to_string();
        next = 0;
        for c in string.chars() {
            next = next + c.to_digit(10).unwrap().pow(2) as i32;
        }
    }
    if next ==1 {
        return true;
    }
    else {
        return false;
    }
}
fn main() {
    let n = 19;
    let result = is_happy(n);
    println!("The number {} is happy: {}", n, result);
}

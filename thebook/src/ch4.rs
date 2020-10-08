pub fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s2: String = s1.clone();
    ext(&mut s2);

    println!("s1 {} s2 {}", s1, s2);

    let long_str = String::from("zero one two three four !");
    for i in 0..8 {
        println!("{}.: {}", i, nth_word(&long_str, i));
    }
}

/// Returns the nth space delimited word from a given string s.
/// ```
/// let s = String::from("hello world");
/// assert_eq!(nth_word(s, 0), "hello");
/// assert_eq!(nth_word(s, 1), "world");
/// assert_eq!(nth_word(s, 2), "");
/// ```
fn nth_word(s: &str, nth: usize) -> &str {
    match nth {
        0 => first_word(s),
        nth => nth_word(skip_first_word(s), nth - 1),
    }
}
/// Returns everything before the first space in s.
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(pos) => &s[..pos],
        None => s,
    }
}

/// Returns everything after the first space in s.
fn skip_first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(pos) => &s[pos + 1..], // + 1 to skip that space
        None => "",
    }
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

fn ext(s: &mut String) {
    s.push_str("tüdelü");
}

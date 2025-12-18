fn main() {
    let mut s = String::from("hello world");

    let s2 = first_word(&s);
    println!("{s2}");

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
}
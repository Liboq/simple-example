fn main() {
    let my_string = String::from("Hello World");
    let word_index = first_word(&my_string[..]);
    println!("{}", word_index)
}
// 获取第一个单词
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

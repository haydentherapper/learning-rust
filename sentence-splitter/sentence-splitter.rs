#![feature(core)]
#![feature(collections)]
#![feature(unicode)]

fn split_sentences(text: &str) {
    let mut pos = 0;
    let delims: &[char] = &['.', '!', '?'];
    let titles: &[&str] = &["Mr.", "Mrs.", "Dr.", "Jr."];

    for i in range(0, text.len()) {
        let c: char = text.char_at(i);
        if delims.contains(&c) {
            // Period followed by whitespace and lowercase
            if i < text.len() - 2 && text.char_at(i + 1).is_whitespace() && text.char_at(i + 2).is_lowercase() {
                continue
            }
            // Period followed by digit
            if i < text.len() - 1 && text.char_at(i + 1).is_digit(10) {
                continue
            }
            // Period surrounded by not whitespace
            if i < text.len() - 1 && i > 0 && !text.char_at(i + 1).is_whitespace() && !text.char_at(i - 1).is_whitespace() {
                continue
            }
            // Period followed by comma or period
            if i < text.len() - 1 && (text.char_at(i + 1) == ',' || text.char_at(i + 1) == '.') {
                continue
            }
            // Period preceded by a title
            let slice1: &str = &text[i - 3..i+1];
            let slice2: &str = &text[i - 2..i+1];
            if titles.contains(&slice1) || titles.contains(&slice2) {
                continue
            }

            println!("{}", text[pos..i + 1].trim_matches(' '));
            pos = i + 1
        }
    }
}

fn main() {
    split_sentences("Hello Mr. Man. Today, why, what a great day today is! I'm sure you'd agree, ay Mr.?");
}
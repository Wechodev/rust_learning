struct ImportantExcerpt<'b> {
    part: &'b str,
}

fn first_word(s: &str) ->&str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item  == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");

    let i = ImportantExcerpt{ part: first_sentence};

}

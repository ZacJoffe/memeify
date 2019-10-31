fn main() {
    println!("{}", meme_generator(String::from("meme")));
}

fn meme_generator(s: String) -> String {
    let mut meme: String = String::from("");

    for c in s.chars() {
        meme.push(c);
        meme.push(' ');
    }

    String::from(&meme[..meme.len() - 1])
}

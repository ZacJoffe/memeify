use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let string_args: String = args.join(" ");

    println!("{}", meme_generator(string_args));
}

fn meme_generator(s: String) -> String {
    let mut meme: String = String::from("");

    for c in s.chars() {
        meme.push(c);
        meme.push(' ');
    }

    String::from(&meme[..meme.len() - 1])
}

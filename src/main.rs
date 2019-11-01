extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Memeify")
        .version("0.1.0")
        .author("Zac J. <zacharyjoffe@gmail.com>")
        .about("Memes strings")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .help("Memeifies the contents of a file")
            .takes_value(true),
        )
        .arg(Arg::with_name("string")
            .short("s")
            .long("string")
            .help("Memeifies a string")
            .takes_value(true),
        ).get_matches();

    if let Some(s) = matches.value_of("string") {
        let args: String = s.to_string();
        println!("{}", meme_generator(args))
    }

    /*
    let mut args: Vec<String> = env::args().collect();
    // remove the first element of the vector ($0)
    args = args[1..].to_vec();
    let string_args: String = args.join(" ");

    println!("{}", meme_generator(string_args));
    */
}

fn meme_generator(s: String) -> String {
    let mut meme: String = String::from("");

    for c in s.chars() {
        meme.push(c);
        meme.push(' ');
    }

    String::from(&meme[..meme.len() - 1])
}

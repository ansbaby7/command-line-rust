use clap::{Command, Arg, ArgAction};


fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Ans Baby")
        .about("An echo clone written in Rust")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

        let text: Vec<String> = matches.get_many::<String>("text").unwrap().map(String::to_string).collect();
        let omit_newline = matches.get_flag("omit_newline");

        let end = if omit_newline {""} else {"\n"};

        print!("{}{}", text.join(" "), end);


}

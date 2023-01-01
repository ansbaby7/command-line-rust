use std::{error::Error, io::{BufRead, BufReader, self}, fs::File};
use clap::{Command, Arg, ArgAction};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Ans Baby")
        .about("A cat clone written in Rust")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s) to concatenate")
                .default_value("-")
                .action(ArgAction::Append),  
        )
        .arg(
            Arg::new("number_lines")
            .short('n')
            .long("number")
            .help("Number lines in output")
            .conflicts_with("number_nonblank")
            .action(ArgAction::SetTrue)

        )
        .arg(
            Arg::new("number_nonblank")
            .short('b')
            .long("number-nonblank")
            .help("Number nonblank lines in output")
            .action(ArgAction::SetTrue)
        )
        .get_matches();
        
    Ok(Config {
        files: matches.get_many::<String>("files").unwrap().map(String::to_string).collect::<Vec<_>>(),
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank"),
    })

}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    let mut line_num = 0;
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                for  line in file.lines() {
                    let line = line?;
                    if config.number_lines{
                        line_num += 1;
                        println!("{:>6}\t{}",line_num, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            line_num += 1;
                            println!("{:>6}\t{}",line_num, line);                            
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}


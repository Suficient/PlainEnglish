use std::env;
use std::process;
use std::fs;
mod utils;
// use std::error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err | {
        println!("Error parsing arguments: {}", err);
        process::exit(1);
    } );

    println!("filename is {}", config.filename);

    let contents = fs::read_to_string(config.filename)
    .expect("could not read file");

    println!("file contents are: {}", contents);

    tokenise(&contents);

}

struct Config{
    filename: String
}


impl Config{
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3{
            return Err("not enough arguments");
        }
    
        let filename = args[2].clone();
        
        Ok(Config{filename})
    }
}


enum TokenType{
    Return,
    IntegerLiteral,
    Semicolon
}

struct Token{
    tokentype: TokenType,
    value: String
}

fn tokenise(contents: &String){
    let mut buffer: String = Default::default();
    for (i, c) in contents.chars().enumerate(){
        println!("{}",c);
        if c.is_ascii_alphabetic(){
            buffer.push(c);
        }
    };
}
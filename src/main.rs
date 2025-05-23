use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();  //显式放到Vec中
    println!("{:?}", args);

    let config = parse_config(&args);

    println!("Searching for {}, in file {}",config.query, config.filename);


    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With test:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config{query, filename}
}

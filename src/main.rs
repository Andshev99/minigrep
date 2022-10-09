use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let config = Config::new(&args);

    println!("Поиск {}", config.query);
    println!("В файле {}", config.filename);
    let contents =
        fs::read_to_string(config.filename).expect("Что-то пошло не так при чтении файла");
    println!("С текстом:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}

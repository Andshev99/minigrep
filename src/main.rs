use std::env;
use std::fs;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("Проблема при разборе аргументов: {}", e);
        process::exit(1);
    });

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
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("недостаточно аргументов");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

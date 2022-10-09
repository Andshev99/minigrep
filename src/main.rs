use minigrep::Config;
use std::env;
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
    if let Err(e) = minigrep::run(config) {
        println!("Ошибка в приложении: {}", e);
        process::exit(1);
    }
}

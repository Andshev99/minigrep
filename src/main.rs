use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let query = &args[1];
    let filename = &args[2];
    println!("Поиск {}", query);
    println!("В файле {}", filename);
    let contents = fs::read_to_string(filename).expect("Что-то пошло не так при чтении файла");
    println!("С текстом:\n{}", contents);
}

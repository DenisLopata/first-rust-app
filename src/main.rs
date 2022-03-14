//// use regex::Regex;
use std::path::Path;
// use rand::Rng;
// use std::cmp::Ordering;
use std::fs;
use std::io;

fn main() {
    // // let re = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    // // println!("Hello, world! {}", re.is_match("hello#_world"));

    // println!("Guess the number!");

    // let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secrent number is: {}", secret_number);
    // loop {
    //     println!("Please input your guess.");

    //     let mut guess = String::new();

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //     println!("You guessed: {}", guess);

    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }

    //original file
    let mut original_file_path = String::new();
    println!("Please input the file path:");
    io::stdin()
        .read_line(&mut original_file_path)
        .expect("Failed to read line");

    let path = Path::new(original_file_path.trim());

    println!("Path is: {}", path.display());
    println!("Parent path is: {}", path.parent().unwrap().display());

    //read parent dir
    let parent_paths = fs::read_dir(path.parent().unwrap()).unwrap();
    for path_file in parent_paths {
        println!("{}", path_file.unwrap().path().display());
    }

    //copy to
    let mut copy_file_path = String::new();
    println!("Please input where to copy path:");
    io::stdin()
        .read_line(&mut copy_file_path)
        .expect("Failed to read line");

    //let copy_result = fs::copy("hello.txt", "hello_copy.txt");
    let copy_result = fs::copy(original_file_path.trim(), copy_file_path.trim());
    match copy_result {
        Ok(_) => println!("File copied successfully"),
        Err(error) => println!("Error: {}", error),
    }

    //println!("Copy result: {}", copy_result.is_ok());
}

//// use regex::Regex;
use std::path::Path;
//use rand::Rng;
// use std::cmp::Ordering;
use std::fs;
//use std::io;
use dialoguer::Confirm;
use dialoguer::MultiSelect;

mod file_opretaions;

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
    if Confirm::new().with_prompt("Do you want to continue?").interact().unwrap() {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }

    // let items = vec!["Option 1", "Option 2"];
    // let chosen : Vec<usize> = MultiSelect::new()
    //     .items(&items)
    //     .interact().unwrap();

    //     for item in chosen {
    //         println!("You chose: {}", items[item]);
    //     }


    //original file
    let mut original_file_path = String::new();
    // println!("Please input the file path:");
    // io::stdin()
    //     .read_line(&mut original_file_path)
    //     .expect("Failed to read line");

        original_file_path = file_opretaions::file_opretaions::read_original_file_path(
        &original_file_path, "Please input the file path:"
    );
    println!("original_file_path is: {}", original_file_path);
    let path = Path::new(original_file_path.trim());

    //read directory content
    let folder_content = fs::read_dir(path).unwrap();

    let mut files: Vec<String> = Vec::new();

    for file in folder_content {
        files.push(file.unwrap().path().display().to_string());
    }

    let chosen : Vec<usize> = MultiSelect::new()
    .items(&files)
    .interact().unwrap();

    for item in chosen {
        println!("You chose: {}", files[item]);
    }
    //println!("Path is: {}", path.display());
    // println!("Parent path is: {}", path.parent().unwrap().display());

    //read parent dir
    // let parent_paths = fs::read_dir(path.parent().unwrap()).unwrap();
    // for path_file in parent_paths {
    //     println!("{}", path_file.unwrap().path().display());
    // }

    //copy to
    let mut copy_file_path = String::new();
    //println!("Please input where to copy path:");
    copy_file_path = file_opretaions::file_opretaions::read_original_file_path(
        &copy_file_path, "Please input where to copy path:"
    );
    // io::stdin()
    //     .read_line(&mut copy_file_path)
    //     .expect("Failed to read line");

    //let copy_result = fs::copy("hello.txt", "hello_copy.txt");
    let copy_result = fs::copy(original_file_path.trim(), copy_file_path.trim());
    match copy_result {
        Ok(_) => println!("File copied successfully"),
        Err(error) => println!("Error: {}", error),
    }

    //println!("Copy result: {}", copy_result.is_ok());
}

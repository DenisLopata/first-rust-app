use dialoguer::MultiSelect;
use std::fs;
use std::path::Path;

mod file_opretaions;

fn main() {
    //original file
    let mut original_file_path = String::new();


    
    original_file_path = file_opretaions::file_opretaions::read_file_path(
        &original_file_path,
        "Please input the origin folder path:",
    );

    if !original_file_path.ends_with("\\") {
        original_file_path.push('\\');
    }

    println!("original_file_path is: {}", original_file_path);

    let path = Path::new(original_file_path.trim());
    if !path.is_dir() {
        println!("Path is not a directory dum dum");
        println!("I crash now");
    }

    //read directory content
    let folder_content = fs::read_dir(path).unwrap();

    let mut files: Vec<String> = Vec::new();

    for file in folder_content {
        files.push(
            file.unwrap()
                .file_name()
                .to_string_lossy()
                .into_owned()
                .to_string(),
        );
    }

    let chosen: Vec<usize> = MultiSelect::new().items(&files).interact().unwrap();

    //copy to
    let mut copy_file_path = String::new();
    copy_file_path = file_opretaions::file_opretaions::read_file_path(
        &copy_file_path,
        "Please input where to copy path:",
    );

    for item in chosen {
        //build original file path including file name
        let full_original_path = path.join(files[item].trim());

        //build copy file path including file name
        let new_file_path: String =
            [copy_file_path.to_string(), files[item].to_string()].join("\\");

        let copy_result = fs::copy(full_original_path, new_file_path.trim());
        match copy_result {
            Ok(_) => println!("File copied successfully"),
            Err(error) => println!("Error: {}", error),
        }
    }

    file_opretaions::file_opretaions::wait_user_input_message("We done! We the best!");
}

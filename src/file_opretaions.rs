
pub mod file_opretaions {

    pub fn read_original_file_path(original_file_path: &String, message: &str) -> String {
        use std::io;
        let mut path: String = original_file_path.trim().to_string();

        println!("{}", message);
        io::stdin()
            .read_line(&mut path)
            .expect("Failed to read line");
        path.trim().to_string()
    }
}

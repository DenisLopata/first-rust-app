
pub mod file_opretaions {
    use std::io;

    pub fn read_file_path(original_file_path: &String, message: &str) -> String {
        
        let mut path: String = original_file_path.trim().to_string();

        println!("{}", message);
        io::stdin()
            .read_line(&mut path)
            .expect("Failed to read line");
        path.trim().to_string()
    }

    pub fn wait_user_input_message(message: &str) {
        println!("{}", message);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }
}

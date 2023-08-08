use std::fs;

pub fn get_input(file_path: &str) -> String {
    let file = fs::read_to_string(file_path);

    match file {
        Ok(file) => file,
        Err(error) => panic!("Couldn`t open file {file_path}, error: {error}",),
    }
}

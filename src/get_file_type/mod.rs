pub fn get_file_type(path: &str) -> String {
    path.split(".").last().unwrap().to_string()
}

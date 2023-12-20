pub mod read {
  pub fn read_string_from_file(path: &str) -> String {
    let file = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("Could not open or find file: {}\nError:{}", path, e));
    return file.to_string();
  }
}

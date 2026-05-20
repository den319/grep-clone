use std::path::Path;


pub fn search(query:&str, contents: &str, path: &Path) -> Vec<(usize, String, String)> {
    let mut results= Vec::new();

    for (idx, line) in contents.lines().enumerate() {
        if line.contains(query) {
            results.push((idx, line.to_string(), path.display().to_string()))
        }
    }

    results
}

pub fn search_case_insensitive(query: &str, contents: &str, path: &Path) -> Vec<(usize, String, String)> {
    let mut results= Vec::new();
    let morphed_query= query.to_lowercase();

    for (idx, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&morphed_query) {
            results.push((idx, line.to_string(), path.display().to_string()))
        }
    }

    results
}

pub fn is_binary(contents: &[u8]) -> bool {
    contents.contains(&0)
}
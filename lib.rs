pub fn search(query: &str, text: &str) {
    if text.contains(query) { println!("Found: {}", text); }
}
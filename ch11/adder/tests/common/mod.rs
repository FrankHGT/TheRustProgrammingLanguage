// any subdir in tests will not be seen as a independent crate in Rust
// use it when you need a common utils in autotest
pub fn setup() {
    println!("setup");
}
use std::fs::OpenOptions;
use std::io::Write;

#[test]
fn write_to_file() {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("test_output.txt")
        .expect("Failed to open test_output.txt");

    file.write_all("Hello from test!\n".as_bytes())
        .expect("Could not write to test_output.txt");
}
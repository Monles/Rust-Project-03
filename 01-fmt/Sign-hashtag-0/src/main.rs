fn main() {
    // https://doc.rust-lang.org/std/fmt/
    println!("Hello {:^15}!", format!("{:?}", Some("hi"))); 
    println!("Hello {:+}!", 5); 
    println!("{:#x}!", 27); 
    println!("{:#o}!", 27); 
    println!("{:#b}!", 27); 
    println!("{:#010x}!", 27);

}

const FORMAT: &str = "Value: {}";


fn main() {

    // =====================================
    // string literals - Compile Time
    // =====================================

    // These are all known at compile time
    println!("Static text");
    println!("Number: {}", 42);
    println!("Hello {}!", "world");

    // Direct string literal
    println!("Value: {}", 123);  // ✅ Works

    // Using const/variable as an argument (not format string)
    const MESSAGE: &str = "Hello";
    println!("Message: {}", MESSAGE);  // ✅ Works

    // Use the const as a regular argument, not as format string
    println!("Format string: {}, Value: {}", FORMAT, 123);
    // Output: Format string: Value: {}, Value: 123

    // Build the string at runtime
    let message = FORMAT.replace("{}", &123.to_string());
    println!("{}", message);
    // Output: Value: 123


    // =====================================
    // String Objects - Runtime
    // =====================================

    // These are created at runtime
    let name = String::from("Alice");
    let greeting = format!("Hello, {}!", name);
    println!("{}", greeting);  
    // println!(greeting);        // ❌ Error - not a string literal
    

    println!("Type your name: ");  
    let mut input = String::new();
    let user_input = std::io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("You entered: {}", input);

    // =====================================
    // Coversion 
    // =====================================

    // String literal → String object
    let literal = "hello";
    let object = literal.to_string();        // or String::from(literal)

    // String object → string literal (not directly possible)
    let object = String::from("hello");
    let slice: &str = &object;               // You get a &str (string slice)
    // But you can't get a true compile-time string literal from runtime data

}

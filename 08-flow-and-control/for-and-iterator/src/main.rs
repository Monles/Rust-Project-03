fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    // for name in names.iter() {
    //     match name {
    //         &"Ferris" => println!("There is a rustacean among us!"),
    //         // TODO ^ Try deleting the & and matching just "Ferris"
    //         _ => println!("Hello {}", name),
    //     }
    // }
    
     println!("names: {:?}", names);

    for a in names.into_iter() {
        match a {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", a),
        }
    }
    
    // println!("names: {:?}", names);
    // FIXME ^ Comment out this line

}
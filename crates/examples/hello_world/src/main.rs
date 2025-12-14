fn main() {
    println!("Hello, world!");

    let mut msg = String::new();
    println!("Please enter a message");
    std::io::stdin().read_line(&mut msg).expect("Failed to read line");
    println!("Message: {}", msg);
}

pub fn run () {
    println!("Printing from the print.rs file");

    // Basic formatting
    println!("{} is from {}", "Static", "Metaverse");

    // Positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Static", "Metaverse", "code");

    // Named arguments
    println!("{name} likes to {activity} and nothig else lol", name = "Static", activity = "code");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
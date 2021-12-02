pub fn run () {
    let x = 5;
    let z = 10;

    let y = 2.5;

    let is_active: bool = true;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let is_greater: bool = x > z;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, is_active, is_greater, a1, face));
} 
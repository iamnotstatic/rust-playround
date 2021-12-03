use std::mem;

pub fn run () {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    numbers[0] = 10;

    println!("{:?}", numbers);
    println!("Here is {}", numbers[0]);
    println!("Length of array is {}", numbers.len());
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    let slice:&[i32] = &numbers;
    println!("Slice: {:?}", slice);

    
}
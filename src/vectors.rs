pub fn run () {
      let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

      numbers[0] = 10;

      numbers.push(6);

      numbers.pop();

      println!("{:?}", numbers);

      for x in numbers.iter() {
            println!("No: {}", x);
      }

      for x in numbers.iter_mut() {
          *x *= 2;
      }

     println!("Number Mut: {:?}", numbers);
}
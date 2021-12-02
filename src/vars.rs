pub fn run () {
      let name = "Static";
      let mut age = 22;
      println!("Hello i'm {} and i am {}", name, age); 
      age = 23;
      println!("Hello i'm {} and i am {}", name, age); 

     const ID: i32 = 001;
     println!("ID: {}", ID);

      let (my_name, my_age) = ("Static", 23);

      println!("{} is {}", my_name, my_age);
}
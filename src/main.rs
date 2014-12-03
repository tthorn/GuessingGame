

use std::io;
use std::rand;


fn main() {
  let mut upper: uint = 100;
  let mut lower = 1u;
  println!("Welcome to the guessing game!");
  println!("");
  let toGuess = (rand::random::<uint>() % 100u) + 1u;
  //println!("The random number is {}", toGuess);

  loop{
    println!("Please enter a guess between {} and {}: ", lower, upper);
    let input = io::stdin().read_line().ok().expect("Failed to read line.");
    //println!("Your input: {}", input);


    //Typecasting to get int
    let input_num: Option<uint> = from_str(input.as_slice().trim());
    let num = match input_num{
      Some(num) => num,
      None  => {
      println!("Please input a number.")
      return;
    }
   };
  match compare(num, toGuess){
    Less => {
      println!("Guess is too small");
      lower = num;
      },
      Greater => {
        println!("Guess is too large");
        upper = num;
      },
      Equal => {
        println!("You win!");
        return;
      },
    }
  }
}

fn compare(a: uint, b: uint) -> Ordering {
        if a < b { Less }
        else if a > b { Greater }
        else { Equal }
}

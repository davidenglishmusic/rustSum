use std::io;

fn main() {
  println!("INPUT:");
  let mut reader = io::stdin();

  let input = reader.read_line().ok().expect("Failed to read line");
  let input_num: Option<int> = input.trim().parse();

  let num = match input_num {
    Some(num) => num,
    None      => {
      println!("Please input an integer");
      return;
    }
  };

  println!("{}", sum(num));
}

fn sum(x: int) -> int {
  if x < 1 { return x; }

  x + sum(x-1)
}

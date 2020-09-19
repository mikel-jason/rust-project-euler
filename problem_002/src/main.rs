use std::vec::Vec;

fn main() {
  let mut fibonacci = get_fibonacci(4000000);
  fibonacci.retain(|&el| el % 2 == 0);
  let mut iter = fibonacci.iter();
  let mut sum = 0;
  loop {
    match iter.next() {
      Some(val) => sum += val,
      None => break,
    }
  }
  println!("Result is {}", sum);
}

fn get_fibonacci(limit: usize) -> Vec<usize> {
  let mut fibonacci = Vec::new();
  fibonacci.push(1);
  fibonacci.push(2);

  loop {
    let candidate = fibonacci.iter().rev().take(2).sum();
    if candidate > limit {
      return fibonacci;
    } else {
       fibonacci.push(candidate);
    }
  }
}

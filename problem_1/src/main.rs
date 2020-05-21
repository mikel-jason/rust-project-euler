fn main() {
  let result = solve(1000);
  println!("Result is: {}", result);
}

fn solve(limit: usize) -> usize {
  let mut iterator = 1;
  let mut sum = 0;
  loop {
    println!("Iterating {}", iterator);
    if iterator >= limit {
      return sum;
    }
    if (iterator % 3 == 0) || (iterator % 5 == 0) {
        println!("Adding {}", iterator);
      sum += iterator;
    }
    iterator+=1;
  }
}

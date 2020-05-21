fn main() {
  let largest_prime = find_largest_prime_factor(600851475143);
  println!("Result is {}", largest_prime);
}

fn find_largest_prime_factor(number: usize) -> usize {
  let mut observer = number;
  let mut current = 0;
  let mut solution = 1;
  loop {
    if current >= observer {
      break;
    }
    current += 1;
    if observer % current != 0 {
      continue;
    }
    if is_prime(current) {
      solution = current;
      observer /= solution;
    }
  }
  return solution;
}

fn is_prime(number: usize) -> bool {
  let mut div = 1;
  loop {
    if number == div {
      return false;
    }
    if number % div == 0 {
      return true;
    }
    div += 1;
  }
}

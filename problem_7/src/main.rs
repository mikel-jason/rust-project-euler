fn main() {
    let mut count = 0;
    let mut current = 0;
    loop {
        current += 1;
        if is_prime(current) {
            count += 1;

            if count >= 10001 {
                println!("Prime no. {} is {}", count, current);
                return;
            }
        }
    }
}

fn is_prime(number: usize) -> bool {
    if number <= 1 {
        return false;
    }
    for div in 2..number {
        if number % div == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let limit = 2000000;
    let mut sum = 0;
    for num in 2..limit {
        if num % 100000 == 0 {
            println!("Currently at {}", num);
        }
        if is_prime(num) {
            sum += num;
        }
    }
    println!("Result is {}", sum)
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

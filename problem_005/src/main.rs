fn main() {
    let limit: usize = (1..21).product();
    // choosing step size as product of all primes < 20
    let step_size = 2 * 3 * 5 * 7 *11 * 13 * 17 * 19;
    let mut current: usize = step_size;
    loop {
        for div in 2..20 {
            if current % div != 0 {
                break;
            }
            if div >= 19 {
                println!("Result is {}", current);
                return;
            }
        }
        if current >= limit {
            println!("Limit of {} reached", limit);
            return;
        }
        current += step_size;
    }
}

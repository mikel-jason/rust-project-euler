fn main() {
    let range = 1..101;
    let sum = range.clone().sum::<usize>();
    let square_of_sums: usize = sum * sum;
    let mut sum_of_squares: usize = 0;
    range.for_each(|x| sum_of_squares += x * x);

    println!("Sum of squares: {}", sum_of_squares);
    println!("Square of sums: {}", square_of_sums);
    println!("Result is {}", square_of_sums - sum_of_squares)
}

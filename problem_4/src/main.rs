fn main() {
    let mut max_palindrome: usize = 0;
    for x in 100..999 {
        for y in 100..999 {
            let candidate = x * y;
            if candidate > max_palindrome && is_palindrome(candidate.to_string()) {
                max_palindrome = candidate;
            }
        }
    }

    print!("Max palindrome is {}", max_palindrome);
}

fn is_palindrome(word: String) -> bool {
    let reverse = word.chars().rev().collect::<String>();
    return word == reverse;
}

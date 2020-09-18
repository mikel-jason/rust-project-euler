fn main() {
    let mut a: usize = 3;
    let mut b: usize;
    let mut c: usize;

    loop {
        b = a + 1;
        c = b + 1;
        if a + b + c > 1000 {
            break;
        }
        loop {
            c = b + 1;
            if a + b + c > 1000 {
                break;
            }
            loop {
                if a + b + c > 1000 {
                    break;
                }
                if a * a + b * b == c * c && a + b + c == 1000 {
                    print!("Result is {}", a * b * c);
                }
                c += 1;
                // println!("New c: {}", c);
            } //c
            b += 1;
        } //b
        a += 1;
    } //a
}

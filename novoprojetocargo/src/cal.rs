fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let n = 5;
    println!("O fatorial de {} é {}", n, factorial(n));
}

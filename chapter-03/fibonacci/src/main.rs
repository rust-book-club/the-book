fn main() {
    println!("{}", fibonacci(1));
    println!("{}", fibonacci(2));
    println!("{}", fibonacci(3));
    println!("{}", fibonacci(4));
    println!("{}", fibonacci(5));
    println!("{}", fibonacci(6));
    println!("{}", fibonacci(7));
    println!("{}", fibonacci(8));
    println!("{}", fibonacci(9));
    println!("{}", fibonacci(10));
}

// returns the nth Fibonacci number where
//   fibonacci(1) == 1
//   fibonacci(2) == 1
//   fibonacci(n) == fibonacci(n-2) + fibonacci(n-1)
//     ...for all n > 2
fn fibonacci(n: u32) -> u32 {
    if n < 3 {
        1
    } else {
        fibonacci(n-2) + fibonacci(n-1)
    }
}

// source: https://stackoverflow.com/a/59418785/2925434
fn fibonacci_tco(n: u64) -> u64 {
    fn f(n: u64, a: u64, b: u64) -> u64 {
        match n {
            0 => a,
            _ => f(n - 1, a + b, a),
        }
    }
    f(n, 0, 1)
}

// Memoization? Requires a Hash Map
use std::env;

// run with: cargo run -q --bin miniecho -- hello!
fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    println!("{}", args.join(" "));
}

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    for i in 1..args.len() {
        println!("{}", args[i]);
    }
}

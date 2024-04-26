use std::env;
fn main() {
    println!("TMS KeyCmd v0.0.1");
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let arg1 = &args[1];
    println!("Arg 1 = {}", arg1);
}
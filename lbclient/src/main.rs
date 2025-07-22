mod database;
fn main() {
    let mut args = std::env::args();
    let _name = args.next();
    let command = args.next().expect("No args were provided!");

    println!("{}", command);
}
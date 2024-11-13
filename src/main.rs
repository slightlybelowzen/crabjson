fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("usage: crabjson [file ...]");
    }
    let files = &args[1..];
    println!("processing files: {:?}", files);
}

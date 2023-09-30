use std::env;
use uuid::Uuid;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} -n [number]", args[0]);
        std::process::exit(1);
    }

    let count: u32 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid number");
            std::process::exit(1);
        }
    };

    (0..count).for_each(|_| {
        let uuid = Uuid::new_v4();
        println!("{}", uuid);
    });
}

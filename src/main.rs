use clap::Parser;
use rand::Rng;

#[derive(Parser)]
struct Args {
    length: usize,
    count: i32,
}

fn main() {
    let args = Args::parse();

    for i in 1..args.count + 1 {
        let mut password : String = Default::default();

        for _j in 0..args.length {
            let new_char : u8 = rand::thread_rng().gen_range(33..126);

            password.push(new_char as char);
        }

        println!("Password {}: {}", i, password);
    }
}
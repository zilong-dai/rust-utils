use num_bigint::BigUint;
use num_traits::Num;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!(
            "`{}` Usage: please input one or more parameter",
            args[0].split("/").last().unwrap()
        );
    }

    for num_str in args.iter().skip(1) {
        match BigUint::from_str_radix(num_str, 10) {
            Ok(num) => print!("0x{} ", num.to_str_radix(16)),
            Err(e) => print!("{} ", e.to_string().replace(" ", "_")),
        }
    }
    println!();
}

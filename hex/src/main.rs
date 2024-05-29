use std::env;
use num_bigint::BigUint;
use num_traits::Num;


// enum BIGNUM{
//     U64,
//     U128,
//     U256,
//     U384,
// }

// fn calc_bignum(dec_str_len: usize) -> BIGNUM {
//     if dec_str_len < 20 {
//         BIGNUM::U64
//     }else if dec_str_len>=20 && dec_str_len<39 {
//         BIGNUM::U128
//     }else if dec_str_len>=39 && dec_str_len<78{
//         BIGNUM::U256
//     }
//     else if dec_str_len>=78 && dec_str_len<118{
//         BIGNUM::U384
//     }else {
//         unreachable!()
//     }
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!(
            "`{}` Usage: please input one or more parameter",
            args[0].split("/").last().unwrap()
        );
    }

    for num_str in args.iter().skip(1) {

        match BigUint::from_str_radix(num_str, 10){
            Ok(num) => print!("0x{} ", num.to_str_radix(16)),
            Err(e) => print!("{} ", e.to_string().replace(" ", "_")),
        }
    }
    println!();
}
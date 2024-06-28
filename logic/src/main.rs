fn main() {
    let eighteen = 18i128;
    let four = 4i128;
    let three = 3i128;
    let two = 2i128;
    let nine = 9i128;
    let eighty_one = 81i128;
    let eighty_three = 83i128;
    for a in 0..4 {
        for b in 0..4 {
            let q_c = -1;
            // let a = 1;
            // let b = 2;
            let c = a ^ b;
            let w = a * b;

            let f = w
                * (w * (four * w - eighteen * (a + b) + eighty_one) + eighteen * (a * a + b * b)
                    - eighty_one * (a + b)
                    + eighty_three);
            println!("F = {}", f);
            let e = three * (a + b + c) - (two * f);
            let b = q_c * ((nine * c) - three * (a + b));
            println!("B + E = {}", b + e);
        }
    }
}

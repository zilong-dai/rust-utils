use ark_std::rand::rngs::StdRng;
use jf_vdf::minroot::{MinRoot, MinRootElement, MinRootField};
use jf_vdf::VDF;

fn minroot_helper<F: MinRootField>() {
    let start = MinRootElement::new(F::one(), F::one());
    let pp = MinRoot::<F>::setup::<StdRng>(1 << 20, None).unwrap();
    let (output, proof) = MinRoot::<F>::eval(&pp, &start).unwrap();
    assert_eq!(output, proof);
    assert!(MinRoot::<F>::verify(&pp, &start, &output, &proof)
        .unwrap()
        .is_ok());
}

fn main() {
    use std::time::Instant;
    let start = Instant::now();
    minroot_helper::<ark_bls12_381::Fr>();
    let elapsed = start.elapsed();
    println!("Elapsed time for ark_bls12_381::Fr: {:?}", elapsed);
}

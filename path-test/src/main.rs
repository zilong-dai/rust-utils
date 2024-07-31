use std::env;

fn main() {
    // Derived from https://stackoverflow.com/questions/73595435/how-to-get-profile-from-cargo-toml-in-build-rs-or-at-runtime
    // We split on the path separator of the *host* machine, which may be different from
    // `std::path::MAIN_SEPARATOR_STR`.
    let out_dir = env::var("OUT_DIR").expect("CARGO_TARGET_DIR env var not found");
    println!("{:?}", out_dir);
    let out_vec = out_dir
        .split(std::path::MAIN_SEPARATOR)
        .into_iter()
        .collect::<Vec<&str>>();
    println!("{:?}", out_vec);
    // println!("{:?}", env::var_os("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS env var not found"));
}

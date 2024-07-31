extern crate which;

fn main() {
    // first search current working directory, second search PATH
    println!("{:?}", which::which("gcc").unwrap());
}

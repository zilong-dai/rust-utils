use std::path::Path;

extern crate sled;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tree = sled::open("~/welcome-to-sled")?;
    // insert and get, similar to std's BTreeMap
    tree.insert("KEY1", "VAL1")?;
    if let Some(v) = tree.get("KEY1")? {
        assert_eq!(v, sled::IVec::from("VAL1"));
    }

    // range queries
    for kv in tree.range("KEY1".."KEY9") {
        match kv {
            Ok((key, value)) => {
                println!("{:?}: {:?}", key, value);
            }
            Err(error) => {
                eprintln!("Error while iterating over range: {}", error);
            }
        }
    }

    // deletion
    tree.remove(&"KEY1")?;

    // atomic compare and swap
    let _ = tree.compare_and_swap("KEY1", Some("VAL1"), Some("VAL2"))?;

    // block until all operations are stable on disk
    // (flush_async also available to get a Future)
    tree.flush()?;

    Ok(())
}

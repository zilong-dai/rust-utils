use native_db::*;
use native_model::{native_model, Model};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[native_model(id = 1, version = 1)]
#[native_db]
struct Item {
    #[primary_key]
    id: u32,
    #[secondary_key]
    name: String,
}

// Define the models
// The lifetime of the models needs to be longer or equal to the lifetime of the database.
// In many cases, it is simpler to use a static variable but it is not mandatory.
static MODELS: Lazy<Models> = Lazy::new(|| {
    let mut models = Models::new();
    models.define::<Item>().unwrap();
    models
});

fn main() -> Result<(), db_type::Error> {
    // Create a database in memory
    let db = Builder::new().create_in_memory(&MODELS)?;

    // Insert data (open a read-write transaction)
    let rw = db.rw_transaction().unwrap();
    rw.insert(Item {
        id: 1,
        name: "red".to_string(),
    })?;
    rw.insert(Item {
        id: 2,
        name: "green".to_string(),
    })?;
    rw.insert(Item {
        id: 3,
        name: "blue".to_string(),
    })?;
    rw.insert(Item {
        id: 4,
        name: "red".to_string(),
    })?;

    // double insert will overwrite the previous data
    rw.insert(Item {
        id: 3,
        name: "red".to_string(),
    })?;
    rw.commit()?;

    // Open a read-only transaction
    let r = db.r_transaction()?;
    // Retrieve data with id=3
    let retrieve_data: Item = r.get().primary(3_u32)?.unwrap();
    println!("data id='3': {:?}", retrieve_data);
    // Iterate items with name starting with "red"
    for item in r.scan().secondary::<Item>(ItemKey::name)?.start_with("red") {
        println!("data name=\"red\": {:?}", item);
    }

    // Remove data (open a read-write transaction)
    let rw = db.rw_transaction()?;
    rw.remove(retrieve_data)?;
    rw.commit()?;
    Ok(())
}

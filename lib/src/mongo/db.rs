use mongodb::{IndexModel, bson::doc};

use super::{MongoDBClient, DBOptions};

pub fn initialize(options: DBOptions) -> Result<(), ()> {

    let client: MongoDBClient = match MongoDBClient::from_options(&options) {
        Ok(c) => c,
        Err(_) => return Err(()),
    };

    println!("Creating Word index");
    let word_col = client.word_collection();
    let mut word_index = 
    IndexModel::builder()
    .keys(doc!{
        "value": 1,
        "kind": 1,
        "created_at": -1, // Descending index
    })
    .build();
    
    match word_col.create_index(word_index, None) {
        Ok(_) => {},
        Err(_) => return Err(()),
    };

    word_index = 
    IndexModel::builder()
    .keys(doc!{
        "value": 1,
        "created_at": -1, // Descending index
    })
    .build();
    match word_col.create_index(word_index, None) {
        Ok(_) => {},
        Err(_) => return Err(()),
    };

    word_index = 
    IndexModel::builder()
    .keys(doc!{
        "value": 1,
    })
    .build();
    match word_col.create_index(word_index, None) {
        Ok(_) => {},
        Err(_) => return Err(()),
    };

    word_index = 
    IndexModel::builder()
    .keys(doc!{
        "translations.value": 1,
    })
    .build();
    match word_col.create_index(word_index, None) {
        Ok(_) => {},
        Err(_) => return Err(()),
    };

    word_index = 
    IndexModel::builder()
    .keys(doc!{
        "translations.value": 1,
        "created_at": -1, // Descending index
    })
    .build();
    match word_col.create_index(word_index, None) {
        Ok(_) => {},
        Err(_) => return Err(()),
    };

    Ok(())
}
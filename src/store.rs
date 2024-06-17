use std::collections::HashSet;
use std::fs::read_to_string;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

use crate::error::AppError;
use crate::model::Item;

#[derive(Debug)]
pub struct Store {
    store: Vec<Item>,
}

impl Store {
    pub fn new(path: &Path) -> Result<Store, AppError> {
        let json_string = read_to_string(path)?;
        let v: Vec<Item> = serde_json::from_str(&json_string)?;

        Ok(Store { store: v })
    }

    pub fn get_values(&self) -> &Vec<Item> {
        &self.store
    }

    pub fn get_keys(&self) -> HashSet<String> {
        let keys = self.store.iter().fold(HashSet::new(), |mut acc, x| {
            acc.insert(x.name.clone());

            acc
        });

        keys
    }

    pub fn update(&mut self, id: usize, new_price: f32) {
        self.store[id].price = new_price;
    }

    pub fn remove(&mut self, id: usize) -> Item {
        self.store.remove(id)
    }

    pub fn add(&mut self, item: Item) {
        self.store.push(item);
    }

    pub fn save_to_file(&self, file_name: &Path) -> Result<(), AppError> {
        let file = File::create(file_name)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &self.store)?;
        writer.flush()?;
        Ok(())
    }
}

use std::fs::read_to_string;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

use prettytable::{color, Attr};
use prettytable::{Cell, Row, Table};

use serde_json::Value;

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

    pub fn update(&mut self, id: usize, new_price: f32) {
        self.store[id].price = new_price;
    }

    pub fn remove(&mut self, id: usize) -> Item {
        self.store.remove(id)
    }

    pub fn add(&mut self, item: Item) {
        self.store.push(item);
    }

    pub fn show(&self) {
        let mut table = Table::new();
        let mut current_price = 0.0;
        let mut count = 0;

        table.add_row(row!["ID", "NAME", "PRICE", "AMOUNT", "PLATFORM", "VALUE"]);

        for row in self.store.iter() {
            let value = &row.price * &row.amount;

            current_price += value;

            table.add_row(Row::new(vec![
                Cell::new(&count.to_string()),
                Cell::new(&row.name),
                Cell::new(&row.price.to_string()),
                Cell::new(&row.amount.to_string()),
                Cell::new(&row.platform.to_string()),
                Cell::new(&value.to_string()),
            ]));

            count += 1;
        }

        let last = Cell::new(&current_price.to_string())
            .with_style(Attr::BackgroundColor(color::RED))
            .with_style(Attr::Italic(true))
            .with_hspan(4);

        table.add_row(row![last]);

        table.printstd();
    }

    pub fn save_to_file(&self, file_name: &Path) -> Result<(), AppError> {
        let file = File::create(file_name)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &self.store)?;
        writer.flush()?;
        Ok(())
    }
}

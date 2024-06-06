use std::fs::read_to_string;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

use prettytable::{color, Attr};
use prettytable::{Cell, Row, Table};

use serde_json::Value;

use crate::error::AppError;
use crate::input::Platform;
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

    pub fn remove(
        &mut self,
        name: String,
        platform: Platform,
        amount: Option<f32>,
    ) {
        match amount {
            Some(amount) => {
              let mut filtered = self.store.clone()
              .into_iter()
              .filter(|item| item.platform == platform && item.name == name)
              .collect::<Vec<Item>>();

              let mut remain = amount;

              for i in 0..filtered.len() {
                if filtered[i].amount > remain {
                  filtered[i].amount -= remain;
                  break;
                } else if filtered[i].amount < remain {
                  remain -= filtered[i].amount;
                  filtered.remove(i);
                } else {
                  filtered.remove(i);
                  break;
                }
              }

              self.store = filtered;
            }
            None => {
              let filtered = self.store.clone()
                  .into_iter()
                  .filter(|item| item.platform == platform && item.name == name)
                  .collect::<Vec<Item>>();

              self.store = filtered; 
            }
        }
    }

    pub fn add(&mut self, item: Item) {
        self.store.push(item);
    }

    pub fn show(&self) {
        let mut table = Table::new();
        let mut current_price = 0.0;
        table.add_row(row!["NAME", "PRICE", "AMOUNT", "PLATFORM", "VALUE"]);

        for row in self.store.iter() {
            let value = &row.price * &row.amount;

            current_price += value;

            table.add_row(Row::new(vec![
                Cell::new(&row.name),
                Cell::new(&row.price.to_string()),
                Cell::new(&row.amount.to_string()),
                Cell::new(&row.platform.to_string()),
                Cell::new(&value.to_string()),
            ]));
        }

        let final_cell = Cell::new(&current_price.to_string())
            .with_style(Attr::BackgroundColor(color::RED))
            .with_style(Attr::Italic(true))
            .with_hspan(4);

        table.add_row(row![final_cell]);

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

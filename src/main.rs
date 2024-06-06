#[macro_use] extern crate prettytable;

use std::path::Path;

use clap::{Parser};

use error::AppError;
use model::Item;
use serde_json::{Value};
use chrono::prelude::*;

mod model;
mod store;
mod input;
mod error;

use input::{Cli, Commands, Platform};
use store::Store;

fn main() -> Result<(), AppError> {
    let file_name = Path::new("./test.json");
    let mut store = Store::new(file_name)?;
    let cli = Cli::parse();

    match &cli.command {
        Commands::Show => store.show(),
        Commands::Remove(args) => {
            println!("{}, {}, {}", args.platform, args.name, args.amount.unwrap_or(0.0))
        },
        Commands::Add(args) => {
            let date_as_string = Utc::now().to_string();
            let item = Item::new(args.name.clone(), args.amount, args.platform, args.price, date_as_string);

            store.add(item);
        }
    }

    store.save_to_file(file_name)?;

    Ok(())
}

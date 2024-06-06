#[macro_use]
extern crate prettytable;

use std::path::Path;

use clap::Parser;

use chrono::prelude::*;
use error::AppError;
use model::Item;
use serde_json::Value;

mod error;
mod input;
mod model;
mod store;

use input::{Cli, Commands, Platform};
use store::Store;

fn main() -> Result<(), AppError> {
    let file_name = Path::new("./test.json");
    let mut store = Store::new(file_name)?;
    let cli = Cli::parse();

    match &cli.command {
        Commands::Show => store.show(),
        Commands::Price => {}
        Commands::Update(arg) => store.update(arg.id, arg.new_price),
        Commands::Remove(args) => {
            let removed = store.remove(args.id);
            println!("removed id: {:?}", removed);
        }
        Commands::Add(args) => {
            let date_as_string = Utc::now().to_string();
            let item = Item::new(
                args.name.clone(),
                args.amount,
                args.platform,
                args.price,
                date_as_string,
            );

            store.add(item);
        }
    }

    store.save_to_file(file_name)?;

    Ok(())
}

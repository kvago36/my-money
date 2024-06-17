#[macro_use]
extern crate prettytable;

use prettytable::{color, Attr, Cell, Row, Table};

use std::path::Path;

use clap::Parser;
use dotenv::dotenv;

use chrono::prelude::*;
use error::AppError;

mod error;
mod input;
mod model;
mod store;

use input::{Cli, Commands, Platform};
use model::{Item, Rates};
use store::Store;

fn main() -> Result<(), AppError> {
    dotenv().ok();

    let coin_api_token = std::env::var("COIN_API_KEY").expect("COIN_API_KEY must be set.");
    let file_name = std::env::var("FILE_NAME").expect("FILE_NAME must be set.");
    let file_path = Path::new(&file_name);
    let mut store = Store::new(file_path)?;
    let cli = Cli::parse();

    let keys = store.get_keys();
    let ids = keys.into_iter().collect::<Vec<String>>().join(",");
    let coin_url = format!(
        "http://rest.coinapi.io/v1/exchangerate/USDT?filter_asset_id={}&invert=true",
        ids
    );

    let client = reqwest::blocking::Client::new();
    let c = client
        .get(coin_url)
        .header("X-CoinAPI-Key", coin_api_token)
        .send();

    let r: Rates = c.unwrap().json().unwrap();

    println!("{:?}", r);

    // get keys
    // send request
    // update store

    // println!("123 {}, {:?}", coin_api_token, file_path);

    match &cli.command {
        Commands::Show => {
            let items = store.get_values();
            let mut table = Table::new();
            let mut current_price = 0.0;
            let mut count = 0;

            table.add_row(row![
                "ID", "NAME", "AMOUNT", "PRICE", "PLATFORM", "FROM", "TO", "1h"
            ]);

            for item in items {
                let rate = r.rates.iter().find(|x| item.name == x.asset_id_quote);
                let mut price_now = 0.0;

                if let Some(r) = rate {
                    price_now = &item.amount * r.rate;
                }

                let buying_price = &item.amount * &item.price;

                let difference = if price_now == 0.0 {
                    0.0
                } else {
                    price_now - buying_price
                };

                let color = if difference.is_sign_negative() {
                    color::RED
                } else if difference == 0.0 {
                    color::BLACK
                } else {
                    color::GREEN
                };

                current_price += price_now;

                table.add_row(Row::new(vec![
                    Cell::new(&count.to_string()),
                    Cell::new(&item.name),
                    Cell::new(&item.amount.to_string()),
                    Cell::new(&item.price.to_string()),
                    Cell::new(&item.platform.to_string()),
                    Cell::new(&buying_price.to_string()),
                    Cell::new(&price_now.to_string()),
                    Cell::new(&difference.to_string()).with_style(Attr::ForegroundColor(color)),
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
        // Commands::Price => {
        //     let keys = store.get_keys();
        //     let ids = keys.into_iter().collect::<Vec<String>>().join(",");
        //     // let mut keys = vec![];

        //     // for key in prices.keys() {
        //     //     keys.push(key.clone());
        //     // }

        //     // let ids = keys.clone().iter(

        //     let coin_url = format!(
        //         "http://rest.coinapi.io/v1/exchangerate/USDT?filter_asset_id={}&invert=true",
        //         ids
        //     );

        //     let client = reqwest::blocking::Client::new();
        //     let c = client
        //         .get(coin_url)
        //         .header("X-CoinAPI-Key", coin_api_token)
        //         .send();

        //     let r: Rates = c.unwrap().json().unwrap();

        //     println!("{:?}", r);

        //     // let d: Vec<f32> = r
        //     //     .rates
        //     //     .into_iter()
        //     //     .filter_map(|r| prices.get(&r.asset_id_quote).and_then(|p| Some(p * r.rate)))
        //     //     .collect();

        //     // let f: f32 = d.into_iter().sum();
        // }
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

    store.save_to_file(file_path)?;

    Ok(())
}

use clap::{
    builder::PossibleValue, command, Args, Parser, Subcommand, ValueEnum,
};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    Add,
    Show,
    Remove,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum Platform {
    Binance,
    ByBit,
    BitKub,
}

impl ValueEnum for Platform {
    fn value_variants<'a>() -> &'a [Self] {
        &[Platform::Binance, Platform::ByBit, Platform::BitKub]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Platform::Binance => PossibleValue::new("binance").help("binance platform"),
            Platform::ByBit => PossibleValue::new("bybit").help("bybit platform"),
            Platform::BitKub => PossibleValue::new("bitkub").help("bitkub platform"),
        })
    }
}

impl ValueEnum for Mode {
    fn value_variants<'a>() -> &'a [Self] {
        &[Mode::Add, Mode::Show, Mode::Remove]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Mode::Add => PossibleValue::new("add").help("Add item"),
            Mode::Show => PossibleValue::new("show").help("Show list"),
            Mode::Remove => PossibleValue::new("remove").help("Remove from list"),
        })
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show current savings
    Show,
    /// Adds files to myapp
    Add(AddArgs),
    /// Update price
    Update(UpdateArgs),
    /// Remove item from current savings
    Remove(RemoveArgs),
}

#[derive(Args)]
pub struct AddArgs {
    pub name: String,
    pub amount: f32,
    pub price: f32,
    pub platform: Platform,
}

#[derive(Args)]
pub struct UpdateArgs {
    pub id: usize,
    pub new_price: f32,
}

#[derive(Args)]
pub struct RemoveArgs {
    pub id: usize,
}

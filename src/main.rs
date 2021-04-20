#[macro_use]
extern crate prettytable;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use prettytable::Table;
use yahoo::Quote;
// use std::io::{self, Write};

use colored::*;
use std::{env, process::exit};
use tokio_test;
use yahoo_finance_api as yahoo;

struct Performance {
    closed_low: u8,
    closed_high: u8,
}

fn get_quotes(symbol: &str) -> Vec<Quote> {
    let provider = yahoo::YahooConnector::new();
    let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    let end = Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999);
    // returns historic quotes with daily interval
    let resp = tokio_test::block_on(provider.get_quote_history(symbol, start, end)).unwrap();
    let quotes = resp.quotes().unwrap();
    return quotes;
}
fn create_table(quotes: &Vec<Quote>) -> Table {
    let mut table = Table::new();
    table.add_row(row!["timestamp", "open", "close", "low", "high"]);
    for q in quotes.iter() {
        create_rows(q, &mut table)    }
    return table;
}
fn create_rows(quote: &Quote, table: &mut Table) {
    match quote {
        yahoo::Quote {
            timestamp,
            low,
            high,
            close,
            open,
            ..
        } => {
            let low = low.round().to_string();
            let high = high.round().to_string();
            let close = close.round().to_string();
            let open = open.round().to_string();
            let dt = DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(timestamp.to_string().parse::<i64>().unwrap(), 0),
                Utc,
            )
            .to_string();
            if open < close {
                table.add_row(row![dt, open, close.green(), low, high]);
            } else if open == close {
                table.add_row(row![dt, open, close.cyan(), low, high]);
            } else {
                table.add_row(row![dt, open, close.red(), low, high]);
            }
            ()
        }    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 | 1 => {
            println!("ERROR: No symbol");
            exit(0);
        }
        _ => {
            for symbol in args[1..].iter() {
                println!("\n");
                print!("\x1B[2J\x1B[1;1H");
                println!("{}", symbol.cyan());
                let quotes = get_quotes(&symbol);
                let table = create_table(&quotes);
                table.printstd();
            }
        }
    }
}

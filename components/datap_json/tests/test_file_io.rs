// error[E0554]: `#![feature]` may not be used on the stable release channel
// #![feature(std)]

extern crate icu_datap_json;

use std::fs::File;
use std::io::BufReader;
// use std::borrow::Borrow;

use icu_datap_json::JsonDataProvider;
use icu_util::datap;
use icu_util::datap::DataProvider;

#[test]
fn test_read_json() {
    // TODO: Make this path relative to this file instead of package root
    let file = File::open("./tests/testdata/all.json").unwrap();
    let reader = BufReader::new(file);
    let json_data_provider = JsonDataProvider::from_reader(reader).unwrap();
    let data = json_data_provider.load(datap::Request {
        locale: "root".to_string(),
        category: datap::Category::Decimal,
        key: datap::Key::Decimal(datap::decimal::Key::SymbolsV1),
        payload: None
    }).unwrap();
    // let decimal_data = match data.payload.borrow() {
    //     datap::ResponsePayload::Decimal(decimal_data) => decimal_data,
    //     _ => unreachable!()
    // };
    let decimal_data: &datap::decimal::SymbolsV1 = data.unwrap_payload();
    assert_eq!(decimal_data, &datap::decimal::SymbolsV1 {
        zero_digit: '0',
        decimal_separator: ".".to_string(),
        grouping_separator: ",".to_string(),
    });
}

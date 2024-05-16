// 1 file(s).
// File(s) read by the parser:
// FEIERTAG
use std::{collections::HashMap, error::Error, rc::Rc};

use chrono::NaiveDate;

use crate::{
    models::{AutoIncrement, Holiday},
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowParser},
    storage::SimpleResourceStorage,
};

use super::ParsedValue;

pub fn parse() -> Result<SimpleResourceStorage<Holiday>, Box<dyn Error>> {
    println!("Parsing FEIERTAG...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a Holiday instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 10, ExpectedType::String),
            ColumnDefinition::new(12, -1, ExpectedType::String),
        ]),
    ]);
    let file_parser = FileParser::new("data/FEIERTAG", row_parser)?;

    let mut rows = Vec::new();
    let auto_increment = AutoIncrement::new();

    // A for loop is used here, as create_instance must be able to return an error.
    for (_, _, values) in file_parser.parse() {
        rows.push(create_instance(values, &auto_increment)?);
    }

    Ok(SimpleResourceStorage::new(rows))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>, auto_increment: &AutoIncrement) -> Result<Rc<Holiday>, Box<dyn Error>> {
    let date: String = values.remove(0).into();
    let name_translations: String = values.remove(0).into();

    let date = NaiveDate::parse_from_str(&date, "%d.%m.%Y")?;
    let name = parse_name_translations(name_translations);

    Ok(Rc::new(Holiday::new(auto_increment.next(), date, name)))
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn parse_name_translations(name_translations: String) -> HashMap<String, String> {
    name_translations.split('>')
        .filter(|&s| !s.is_empty())
        .map(|s| {
            let mut parts = s.split('<');

            let v = parts.next().unwrap().to_string();
            let k = parts.next().unwrap().to_string();

            (k, v)
        })
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.insert(k, v);
            acc
        })
}

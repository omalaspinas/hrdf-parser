// 1 file(s).
// File(s) read by the parser:
// RICHTUNG
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{Direction, ResourceCollection, ResourceIndex},
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowParser},
    storage::SimpleResourceStorage,
};

use super::ParsedValue;

pub fn parse() -> Result<(SimpleResourceStorage<Direction>, ResourceIndex<Direction, String>), Box<dyn Error>> {
    println!("Parsing RICHTUNG...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a Direction instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::String),
            ColumnDefinition::new(9, -1, ExpectedType::String),
        ]),
    ]);
    let file_parser = FileParser::new("data/RICHTUNG", row_parser)?;

    let rows: ResourceCollection<Direction>;
    let mut legacy_pk_index = HashMap::new();

    rows = file_parser
        .parse()
        .map(|(_, _, values)| {
            let (instance, k) = create_instance(values);
            legacy_pk_index.insert(k, Rc::clone(&instance));
            instance
        })
        .collect();

    Ok((SimpleResourceStorage::new(rows), legacy_pk_index))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>) -> (Rc<Direction>, String) {
    let id_str: String = values.remove(0).into();
    let name: String = values.remove(0).into();

    let id = remove_first_char(&id_str);
    let id = id.parse::<i32>().unwrap();

    (Rc::new(Direction::new(id, name)), id_str)
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn remove_first_char(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.as_str()
}

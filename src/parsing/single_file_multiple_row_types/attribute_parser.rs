// 5 file(s).
// File(s) read by the parser:
// ATTRIBUT
// ---
// Files not used by the parser:
// ATTRIBUT_DE, ATTRIBUT_EN, ATTRIBUT_FR, ATTRIBUT_IT
use std::{collections::HashMap, error::Error, str::FromStr};

use crate::{
    models::{Attribute, Language, Model},
    parsing::{
        AdvancedRowMatcher, ColumnDefinition, ExpectedType, FastRowMatcher, FileParser,
        ParsedValue, RowDefinition, RowParser,
    },
    storage::SimpleResourceStorage,
    utils::AutoIncrement,
};

pub fn parse() -> Result<(SimpleResourceStorage<Attribute>, HashMap<String, i32>), Box<dyn Error>> {
    println!("Parsing ATTRIBUT...");
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;
    const ROW_C: i32 = 3;
    const ROW_D: i32 = 4;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create an Attribute instance.
        RowDefinition::new(ROW_A, Box::new(
            AdvancedRowMatcher::new(r"^.{2} [0-9] [0-9 ]{3} [0-9 ]{2}$")?
        ), vec![
            ColumnDefinition::new(1, 2, ExpectedType::String),
            ColumnDefinition::new(4, 4, ExpectedType::Integer16),
            ColumnDefinition::new(6, 8, ExpectedType::Integer16),
            ColumnDefinition::new(10, 11, ExpectedType::Integer16),
        ]),
        // This row is ignored.
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(1, 1, "#", true)), Vec::new()),
        // This row indicates the language for translations in the section that follows it.
        RowDefinition::new(ROW_C, Box::new(FastRowMatcher::new(1, 1, "<", true)), vec![
            ColumnDefinition::new(1, -1, ExpectedType::String),
        ]),
        // This row contains the description in a specific language.
        RowDefinition::new(ROW_D, Box::new(AdvancedRowMatcher::new(r"^.{2} .+$")?), vec![
            ColumnDefinition::new(1, 2, ExpectedType::String),
            ColumnDefinition::new(4, -1, ExpectedType::String),
        ]),
    ]);
    // The ATTRIBUT file is used instead of ATTRIBUT_* for simplicity's sake.
    let parser = FileParser::new("data/ATTRIBUT", row_parser)?;

    let auto_increment = AutoIncrement::new();
    let mut data = HashMap::new();
    let mut pk_type_converter = HashMap::new();

    let mut current_language = Language::default();

    for (id, _, values) in parser.parse() {
        match id {
            ROW_A => {
                let attribute = create_instance(values, &auto_increment, &mut pk_type_converter);
                data.insert(attribute.id(), attribute);
            }
            ROW_B => {}
            ROW_C => update_current_language(values, &mut current_language),
            ROW_D => set_description(values, &pk_type_converter, &mut data, current_language),
            _ => unreachable!(),
        }
    }

    Ok((SimpleResourceStorage::new(data), pk_type_converter))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(
    mut values: Vec<ParsedValue>,
    auto_increment: &AutoIncrement,
    pk_type_converter: &mut HashMap<String, i32>,
) -> Attribute {
    let designation: String = values.remove(0).into();
    let stop_scope: i16 = values.remove(0).into();
    let main_sorting_priority: i16 = values.remove(0).into();
    let secondary_sorting_priority: i16 = values.remove(0).into();

    let id = auto_increment.next();

    pk_type_converter.insert(designation.to_owned(), id);
    Attribute::new(
        id,
        designation.to_owned(),
        stop_scope,
        main_sorting_priority,
        secondary_sorting_priority,
    )
}

fn set_description(
    mut values: Vec<ParsedValue>,
    pk_type_converter: &HashMap<String, i32>,
    data: &mut HashMap<i32, Attribute>,
    language: Language,
) {
    let legacy_id: String = values.remove(0).into();
    let description: String = values.remove(0).into();

    data.get_mut(&pk_type_converter.get(&legacy_id).unwrap())
        .unwrap()
        .set_description(language, &description);
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn update_current_language(mut values: Vec<ParsedValue>, current_language: &mut Language) {
    let language: String = values.remove(0).into();
    let language = &language[1..&language.len() - 1];

    if language != "text" {
        *current_language = Language::from_str(language).unwrap();
    }
}

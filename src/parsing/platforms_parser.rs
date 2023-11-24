// GLEIS, GLEIS_LV95, GLEIS_WGS
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{JourneyPlatform, Platform},
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowMatcher, RowParser},
};

pub fn load_journey_stop_platforms_and_platforms() -> Result<
    (
        Vec<Rc<JourneyPlatform>>,
        HashMap<(i32, i32), Vec<Rc<JourneyPlatform>>>,
        Vec<Rc<Platform>>,
        HashMap<(i32, i32), Rc<Platform>>,
    ),
    Box<dyn Error>,
> {
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        RowDefinition::new(ROW_A, RowMatcher::new(8, 9, "#", false), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 14, ExpectedType::Integer32),
            ColumnDefinition::new(16, 21, ExpectedType::String),
            // 23-30 with #
            ColumnDefinition::new(24, 30, ExpectedType::Integer32),
            ColumnDefinition::new(32, 35, ExpectedType::OptionInteger16),
            ColumnDefinition::new(37, 42, ExpectedType::OptionInteger32),
        ]),
        RowDefinition::new(ROW_B, RowMatcher::new(8, 9, "#", true), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            // 9-16 with #
            ColumnDefinition::new(10, 16, ExpectedType::Integer32),
            ColumnDefinition::new(18, -1, ExpectedType::String),
        ]),
    ]);
    let file_parser = FileParser::new("data/GLEIS", row_parser)?;

    let mut journey_platform = vec![];
    let mut platforms = vec![];
    let mut first_section_last_cursor_position = 0;

    for (id, bytes_read, mut values) in file_parser.parse() {
        match id {
            ROW_A => {
                first_section_last_cursor_position += bytes_read;

                let stop_id = i32::from(values.remove(0));
                let journey_id = i32::from(values.remove(0));
                let unknown1 = String::from(values.remove(0));
                let platform_index = i32::from(values.remove(0));
                let hour: Option<i16> = values.remove(0).into();
                let bit_field_id: Option<i32> = values.remove(0).into();

                journey_platform.push(Rc::new(JourneyPlatform::new(
                    journey_id,
                    stop_id,
                    unknown1,
                    platform_index,
                    hour,
                    bit_field_id,
                )))
            }
            ROW_B => {
                let stop_id = i32::from(values.remove(0));
                let platform_index = i32::from(values.remove(0));
                let (number, sectors) = parse_platform_data(values.remove(0).into());

                platforms.push(Rc::new(Platform::new(
                    stop_id,
                    platform_index,
                    number,
                    sectors,
                )))
            }
            _ => unreachable!(),
        }
    }

    println!(
        "Size of section A of the GLEIS file in bytes: {}",
        first_section_last_cursor_position
    );
    let journey_platform_index = create_journey_platform_index(&journey_platform);
    let platforms_index = create_platforms_index(&platforms);

    println!("Start parsing GLEIS_LV95...");
    load_lv95_stop_coordinates(&platforms_index)?;

    Ok((
        journey_platform,
        journey_platform_index,
        platforms,
        platforms_index,
    ))
}

fn create_journey_platform_index(
    journey_platform: &Vec<Rc<JourneyPlatform>>,
) -> HashMap<(i32, i32), Vec<Rc<JourneyPlatform>>> {
    journey_platform
        .iter()
        .fold(HashMap::new(), |mut acc, item| {
            acc.entry((*item.journey_id(), *item.stop_id()))
                .or_insert(Vec::new())
                .push(Rc::clone(item));
            acc
        })
}

fn create_platforms_index(platforms: &Vec<Rc<Platform>>) -> HashMap<(i32, i32), Rc<Platform>> {
    platforms.iter().fold(HashMap::new(), |mut acc, item| {
        acc.insert((*item.stop_id(), *item.platform_index()), Rc::clone(item));
        acc
    })
}

fn load_lv95_stop_coordinates(platforms_index: &HashMap<(i32, i32), Rc<Platform>>) -> Result<(), Box<dyn Error>> {
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;
    const ROW_C: i32 = 3;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // TODO : Remove this and do a seek in the file.
        RowDefinition::new(0, RowMatcher::new(8, 9, "#", false), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 14, ExpectedType::Integer32),
            ColumnDefinition::new(16, 21, ExpectedType::String),
            // 23-30 with #
            ColumnDefinition::new(24, 30, ExpectedType::Integer32),
            ColumnDefinition::new(32, 35, ExpectedType::OptionInteger16),
            ColumnDefinition::new(37, 42, ExpectedType::OptionInteger32),
        ]),
        RowDefinition::new(ROW_A, RowMatcher::new(17, 18, "G", true), vec![
            // TODO : Add the possibility of having no values to parse.
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
        ]),
        RowDefinition::new(ROW_B, RowMatcher::new(17, 20, "I A", true), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            // 9-16 with #
            ColumnDefinition::new(10, 16, ExpectedType::Integer32),
            ColumnDefinition::new(22, -1, ExpectedType::String),
        ]),
        RowDefinition::new(ROW_C, RowMatcher::new(17, 18, "K", true), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            // 9-16 with #
            ColumnDefinition::new(10, 16, ExpectedType::Integer32),
            ColumnDefinition::new(20, -1, ExpectedType::String),
        ]),
    ]);
    let file_parser = FileParser::new("data/GLEIS_LV95", row_parser)?;

    for (id, _, mut values) in file_parser.parse() {
        if id == 1 {
            println!("1 {:?}", values);
        }
        if id == 2 {
            println!("2 {:?}", values);
        }
        if id == 3 {
            println!("3 {:?}", values);
        }
    }

    Ok(())
}

fn parse_platform_data(platform_data: String) -> (String, Option<String>) {
    let cleaned_platform_data = platform_data.trim().to_string() + " ";
    let parsed_values = cleaned_platform_data
        .split("' ")
        .fold(HashMap::new(), |mut acc, item| {
            if item.is_empty() {
                // There will always be an empty string as the last element, it is always ignored.
                acc
            } else {
                let tmp: Vec<&str> = item.split(" '").collect();
                acc.insert(tmp[0], tmp[1]);
                acc
            }
        });

    // It should always have a G entry.
    let number = parsed_values.get("G").unwrap().to_string();
    let sectors = parsed_values.get("A").map(|s| s.to_string());

    (number, sectors)
}

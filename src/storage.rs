use std::error::Error;

use chrono::{Days, NaiveDate};
use rustc_hash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        Attribute, BitField, Direction, ExchangeTimeAdministration, ExchangeTimeJourney,
        ExchangeTimeLine, Holiday, InformationText, Journey, JourneyPlatform, Line, Model,
        Platform, Stop, StopConnection, ThroughService, TimetableMetadataEntry, TransportCompany,
        TransportType, Version,
    },
    parsing,
    utils::{count_days_between_two_dates, timetable_end_date, timetable_start_date},
};

// ------------------------------------------------------------------------------------------------
// --- DataStorage
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct DataStorage {
    // Time-relevant data.
    bit_fields: ResourceStorage<BitField>,
    holidays: ResourceStorage<Holiday>,
    timetable_metadata: ResourceStorage<TimetableMetadataEntry>,

    // Master data.
    attributes: ResourceStorage<Attribute>,
    information_texts: ResourceStorage<InformationText>,
    directions: ResourceStorage<Direction>,
    lines: ResourceStorage<Line>,
    transport_companies: ResourceStorage<TransportCompany>,
    transport_types: ResourceStorage<TransportType>,

    // Stop data.
    stops: ResourceStorage<Stop>,
    stop_connections: ResourceStorage<StopConnection>,

    // Timetable data.
    journeys: ResourceStorage<Journey>,
    journey_platform: ResourceStorage<JourneyPlatform>,
    platforms: ResourceStorage<Platform>,
    through_service: ResourceStorage<ThroughService>,

    // Exchange times.
    exchange_times_administration: ResourceStorage<ExchangeTimeAdministration>,
    exchange_times_journey: ResourceStorage<ExchangeTimeJourney>,
    exchange_times_line: ResourceStorage<ExchangeTimeLine>,

    // Maps.
    bit_fields_by_day: FxHashMap<NaiveDate, FxHashSet<i32>>,
    bit_fields_by_stop_id: FxHashMap<i32, FxHashSet<i32>>,
    journeys_by_stop_id_and_bit_field_id: FxHashMap<(i32, i32), Vec<i32>>,
    stop_connections_by_stop_id: FxHashMap<i32, FxHashSet<i32>>,
}

#[allow(unused)]
impl DataStorage {
    pub fn new(version: Version, path: &str) -> Result<Self, Box<dyn Error>> {
        // Time-relevant data.
        let bit_fields = parsing::load_bit_fields(path)?;
        let holidays = parsing::load_holidays(path)?;
        let timetable_metadata = parsing::load_timetable_metadata(path)?;

        // Master data.
        let (attributes, attributes_pk_type_converter) = parsing::load_attributes(path)?;
        let (directions, directions_pk_type_converter) = parsing::load_directions(path)?;
        let information_texts = parsing::load_information_texts(path)?;
        let lines = parsing::load_lines(path)?;
        let transport_companies = parsing::load_transport_companies(path)?;
        let (transport_types, transport_types_pk_type_converter) =
            parsing::load_transport_types(path)?;

        // Stop data.
        let stop_connections = parsing::load_stop_connections(path, &attributes_pk_type_converter)?;
        let stops = parsing::load_stops(version, path)?;

        // Timetable data.
        let (journeys, journeys_pk_type_converter) = parsing::load_journeys(
            path,
            &transport_types_pk_type_converter,
            &attributes_pk_type_converter,
            &directions_pk_type_converter,
        )?;
        let (journey_platform, platforms) =
            parsing::load_platforms(path, &journeys_pk_type_converter)?;
        let through_service = parsing::load_through_service(path, &journeys_pk_type_converter)?;

        // Exchange times.
        let exchange_times_administration = parsing::load_exchange_times_administration(path)?;
        let exchange_times_journey =
            parsing::load_exchange_times_journey(path, &journeys_pk_type_converter)?;
        let exchange_times_line =
            parsing::load_exchange_times_line(path, &transport_types_pk_type_converter)?;

        let bit_fields_by_day = create_bit_fields_by_day(&bit_fields, &timetable_metadata);
        let bit_fields_by_stop_id = create_bit_fields_by_stop_id(&journeys);
        let journeys_by_stop_id_and_bit_field_id =
            create_journeys_by_stop_id_and_bit_field_id(&journeys);
        let stop_connections_by_stop_id = create_stop_connections_by_stop_id(&stop_connections);

        let mut data_storage = Self {
            // Time-relevant data.
            bit_fields,
            holidays,
            timetable_metadata,
            // Master data.
            attributes,
            information_texts,
            directions,
            lines,
            transport_companies,
            transport_types,
            // Stop data.
            stop_connections,
            stops,
            // Timetable data.
            journeys,
            journey_platform,
            platforms,
            through_service,
            // Exchange times.
            exchange_times_administration,
            exchange_times_journey,
            exchange_times_line,
            // Maps.
            bit_fields_by_day,
            bit_fields_by_stop_id,
            journeys_by_stop_id_and_bit_field_id,
            stop_connections_by_stop_id,
        };

        Ok(data_storage)
    }

    pub fn bit_fields(&self) -> &ResourceStorage<BitField> {
        &self.bit_fields
    }

    pub fn journeys(&self) -> &ResourceStorage<Journey> {
        &self.journeys
    }

    pub fn lines(&self) -> &ResourceStorage<Line> {
        &self.lines
    }

    pub fn platforms(&self) -> &ResourceStorage<Platform> {
        &self.platforms
    }

    pub fn stop_connections(&self) -> &ResourceStorage<StopConnection> {
        &self.stop_connections
    }

    pub fn stops(&self) -> &ResourceStorage<Stop> {
        &self.stops
    }

    pub fn timetable_metadata(&self) -> &ResourceStorage<TimetableMetadataEntry> {
        &self.timetable_metadata
    }

    pub fn bit_fields_by_day(&self) -> &FxHashMap<NaiveDate, FxHashSet<i32>> {
        &self.bit_fields_by_day
    }

    pub fn bit_fields_by_stop_id(&self) -> &FxHashMap<i32, FxHashSet<i32>> {
        &self.bit_fields_by_stop_id
    }

    pub fn journeys_by_stop_id_and_bit_field_id(&self) -> &FxHashMap<(i32, i32), Vec<i32>> {
        &self.journeys_by_stop_id_and_bit_field_id
    }

    pub fn stop_connections_by_stop_id(&self) -> &FxHashMap<i32, FxHashSet<i32>> {
        &self.stop_connections_by_stop_id
    }
}

// ------------------------------------------------------------------------------------------------
// --- ResourceStorage
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceStorage<M: Model<M>> {
    data: FxHashMap<M::K, M>,
}

impl<M: Model<M>> ResourceStorage<M> {
    pub fn new(data: FxHashMap<M::K, M>) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &FxHashMap<M::K, M> {
        &self.data
    }

    pub fn find(&self, k: M::K) -> &M {
        &self.data().get(&k).unwrap()
    }

    pub fn entries(&self) -> Vec<&M> {
        self.data.values().collect()
    }

    pub fn resolve_ids(&self, ids: &FxHashSet<M::K>) -> Vec<&M> {
        ids.iter().map(|&id| self.find(id)).collect()
    }
}

// ------------------------------------------------------------------------------------------------
// --- Maps
// ------------------------------------------------------------------------------------------------

fn create_bit_fields_by_day(
    bit_fields: &ResourceStorage<BitField>,
    timetable_metadata: &ResourceStorage<TimetableMetadataEntry>,
) -> FxHashMap<NaiveDate, FxHashSet<i32>> {
    let start_date = timetable_start_date(timetable_metadata);
    let num_days = count_days_between_two_dates(start_date, timetable_end_date(timetable_metadata));

    let dates: Vec<NaiveDate> = (0..num_days)
        .into_iter()
        .map(|i| {
            start_date
                .checked_add_days(Days::new(i.try_into().unwrap()))
                .unwrap()
        })
        .collect();

    let mut map = FxHashMap::default();
    dates.iter().for_each(|date| {
        map.entry(*date).or_insert(FxHashSet::default()).insert(0);
    });

    bit_fields.data().keys().fold(map, |mut acc, bit_field_id| {
        let bit_field = bit_fields.find(*bit_field_id);
        let indexes: Vec<usize> = bit_field
            .bits()
            .iter()
            // The first two bits must be ignored.
            .skip(2)
            .enumerate()
            .filter(|(i, &x)| *i < num_days && x == 1)
            .map(|(i, _)| i)
            .collect();

        indexes.into_iter().for_each(|i| {
            acc.entry(dates[i])
                .or_insert(FxHashSet::default())
                .insert(bit_field.id());
        });

        acc
    })
}

fn create_bit_fields_by_stop_id(
    journeys: &ResourceStorage<Journey>,
) -> FxHashMap<i32, FxHashSet<i32>> {
    journeys
        .entries()
        .into_iter()
        .fold(FxHashMap::default(), |mut acc, journey| {
            journey.route().iter().for_each(|route_entry| {
                acc.entry(route_entry.stop_id())
                    .or_insert(FxHashSet::default())
                    .insert(journey.bit_field_id().unwrap_or(0));
            });
            acc
        })
}

fn create_journeys_by_stop_id_and_bit_field_id(
    journeys: &ResourceStorage<Journey>,
) -> FxHashMap<(i32, i32), Vec<i32>> {
    journeys
        .entries()
        .into_iter()
        .fold(FxHashMap::default(), |mut acc, journey| {
            journey.route().iter().for_each(|route_entry| {
                acc.entry((route_entry.stop_id(), journey.bit_field_id().unwrap_or(0)))
                    .or_insert(Vec::new())
                    .push(journey.id());
            });
            acc
        })
}

fn create_stop_connections_by_stop_id(
    stop_connections: &ResourceStorage<StopConnection>,
) -> FxHashMap<i32, FxHashSet<i32>> {
    stop_connections
        .data()
        .values()
        .fold(FxHashMap::default(), |mut acc, stop_connection| {
            acc.entry(stop_connection.stop_id_1())
                .or_insert(FxHashSet::default())
                .insert(stop_connection.id());
            acc
        })
}

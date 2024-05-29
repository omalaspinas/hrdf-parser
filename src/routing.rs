use std::collections::HashSet;

use chrono::NaiveDate;

use crate::{
    hrdf::Hrdf,
    models::{Journey, Model, Time},
    storage::DataStorage,
};

#[derive(Debug, Clone)]
struct RouteSection {
    journey_id: i32,
    departure_stop_id: i32,
    arrival_stop_id: i32,
}

impl RouteSection {
    pub fn new(journey_id: i32, departure_stop_id: i32, arrival_stop_id: i32) -> Self {
        Self {
            journey_id,
            departure_stop_id,
            arrival_stop_id,
        }
    }

    // Getters/Setters

    pub fn journey_id(&self) -> i32 {
        self.journey_id
    }

    pub fn departure_stop_id(&self) -> i32 {
        self.departure_stop_id
    }

    pub fn arrival_stop_id(&self) -> i32 {
        self.arrival_stop_id
    }

    pub fn set_arrival_stop_id(&mut self, value: i32) {
        self.arrival_stop_id = value;
    }

    // Functions

    pub fn journey<'a>(&'a self, data_storage: &'a DataStorage) -> &Journey {
        data_storage.journeys().find(self.journey_id())
    }
}

#[derive(Debug, Clone)]
struct Node {
    route_sections: Vec<RouteSection>,
    visited_stops: HashSet<i32>,
}

impl Node {
    pub fn new(route_sections: Vec<RouteSection>, visited_stops: HashSet<i32>) -> Self {
        Self {
            route_sections,
            visited_stops,
        }
    }

    pub fn route_sections(&self) -> &Vec<RouteSection> {
        &self.route_sections
    }

    pub fn visited_stops(&self) -> &HashSet<i32> {
        &self.visited_stops
    }

    // Functions

    pub fn arrival_time<'a>(&'a self, data_storage: &'a DataStorage) -> &Time {
        let route_section = self.route_sections().last().unwrap();

        route_section
            .journey(data_storage)
            .route()
            .iter()
            .find(|route_entry| route_entry.stop_id() == route_section.arrival_stop_id())
            .unwrap()
            .arrival_time()
            .as_ref()
            // TODO: it could crash here.
            .unwrap()
    }

    pub fn print(&self, data_storage: &DataStorage) {
        for route_section in self.route_sections() {
            let journey = route_section.journey(data_storage);
            println!("Journey #{}", journey.id());

            let mut route_iter = journey.route().into_iter().peekable();

            while route_iter.peek().unwrap().stop_id() != route_section.departure_stop_id() {
                route_iter.next();
            }

            let mut route = Vec::new();

            loop {
                route.push(route_iter.next().unwrap());

                if route.last().unwrap().stop_id() == route_section.arrival_stop_id() {
                    break;
                }
            }

            for (i, route_entry) in route.iter().enumerate() {
                let arrival_time = if i == 0 {
                    " ".repeat(5)
                } else {
                    format!("{}", route_entry.arrival_time().as_ref().unwrap())
                };

                let departure_time = if i == route.len() - 1 {
                    " ".repeat(5)
                } else {
                    format!("{}", route_entry.departure_time().as_ref().unwrap())
                };

                let stop = route_entry.stop(data_storage);

                println!(
                    "  {} {: <36} {} - {} ; {}",
                    stop.id(),
                    stop.name(),
                    arrival_time,
                    departure_time,
                    stop.transfer_flag(),
                );
            }
        }
    }
}

impl Hrdf {
    pub fn plan_journey(
        &self,
        departure_stop_id: i32,
        target_arrival_stop_id: i32,
        departure_date: NaiveDate,
        departure_time: Time,
    ) {
        let mut nodes = self.get_initial_nodes(
            departure_stop_id,
            target_arrival_stop_id,
            departure_date,
            departure_time,
        );
        let mut next_nodes: Vec<Node> = Vec::new();

        let mut solution: Option<Node> = None;

        while !nodes.is_empty() {
            println!("{}", nodes.len());
            while !nodes.is_empty() {
                let parent_node = nodes.remove(0);
                let route_section = parent_node.route_sections().last().unwrap();
                println!("A");

                if route_section.arrival_stop_id() == target_arrival_stop_id {
                    if let Some(solution_ref) = solution.as_ref() {
                        let t1 = parent_node.arrival_time(self.data_storage());
                        let t2 = solution_ref.arrival_time(self.data_storage());

                        if t1 < t2 {
                            solution = Some(parent_node);
                        }
                    } else {
                        solution = Some(parent_node);
                    }

                    continue;
                }

                self.create_node(
                    &parent_node,
                    route_section.journey_id(),
                    route_section.arrival_stop_id(),
                    target_arrival_stop_id,
                )
                .map(|node| {
                    nodes.push(node);
                });
            }

            next_nodes = nodes;
            nodes = Vec::new();
        }

        if let Some(solution) = solution {
            solution.print(self.data_storage());
        }
    }

    fn get_operating_journeys(&self, date: NaiveDate, stop_id: i32) -> Vec<&Journey> {
        let data_storage = self.data_storage();

        let journeys_1 = data_storage.journeys().find_by_day(date);
        let journeys_2 = data_storage.journeys().find_by_stop_id(stop_id);

        let ids: HashSet<i32> = journeys_1.intersection(&journeys_2).cloned().collect();

        data_storage.journeys().resolve_ids(data_storage, ids)
    }

    fn next_departures(&self, stop_id: i32, date: NaiveDate, time: Time) -> Vec<&Journey> {
        let mut journeys: Vec<&Journey> = self.get_operating_journeys(date, stop_id);

        journeys.sort_by(|a, b| {
            let a = self.get_departure_time(a, stop_id);
            let b = self.get_departure_time(b, stop_id);
            a.cmp(b)
        });

        journeys
            .into_iter()
            .filter(|j| self.get_departure_time(j, stop_id) >= &time)
            .take(5)
            .collect()
    }

    fn get_departure_time<'a>(&'a self, journey: &'a Journey, stop_id: i32) -> &Time {
        journey
            .route()
            .iter()
            .find(|route_entry| route_entry.stop_id() == stop_id)
            .unwrap()
            .departure_time()
            .as_ref()
            // TODO: it could crash here.
            .unwrap()
    }

    fn get_next_route_section(
        &self,
        journey: &Journey,
        departure_stop_id: i32,
        target_arrival_stop_id: i32,
    ) -> Option<(RouteSection, HashSet<i32>)> {
        let mut route_iter = journey.route().iter().peekable();

        while route_iter.peek().unwrap().stop_id() != departure_stop_id {
            route_iter.next();
        }

        route_iter.next();
        let mut visited_stops = HashSet::new();

        while route_iter.peek().is_some() {
            let stop = self
                .data_storage()
                .stops()
                .find(route_iter.peek().unwrap().stop_id());
            visited_stops.insert(stop.id());

            if stop.transfer_flag() != 0 || stop.id() == target_arrival_stop_id {
                return Some((
                    RouteSection::new(journey.id(), departure_stop_id, stop.id()),
                    visited_stops,
                ));
            }

            route_iter.next();
        }

        None
    }

    fn get_initial_nodes(
        &self,
        departure_stop_id: i32,
        target_arrival_stop_id: i32,
        departure_date: NaiveDate,
        departure_time: Time,
    ) -> Vec<Node> {
        self.next_departures(departure_stop_id, departure_date, departure_time)
            .iter()
            .filter_map(|journey| {
                self.get_next_route_section(journey, departure_stop_id, target_arrival_stop_id)
                    .map(|(route_section, visited_stops)| {
                        Node::new(vec![route_section], visited_stops)
                    })
            })
            .collect()
    }

    fn create_node(
        &self,
        parent_node: &Node,
        journey_id: i32,
        departure_stop_id: i32,
        target_arrival_stop_id: i32,
    ) -> Option<Node> {
        self.get_next_route_section(
            self.data_storage().journeys().find(journey_id),
            departure_stop_id,
            target_arrival_stop_id,
        )
        .map(|(route_section, new_visited_stops)| {
            let mut route_sections = parent_node.route_sections().clone();

            if route_sections.last().unwrap().journey_id() == journey_id {
                route_sections
                    .last_mut()
                    .unwrap()
                    .set_arrival_stop_id(route_section.arrival_stop_id());
            } else {
                route_sections.push(route_section);
            }

            let mut visited_stops = parent_node.visited_stops().clone();
            visited_stops.extend(new_visited_stops);

            Node::new(route_sections, visited_stops)
        })
    }
}

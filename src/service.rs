use std::{str::FromStr, sync::Arc};

use axum::{extract::Query, routing::get, Json, Router};
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    hrdf::Hrdf,
    isochrone::{IsochroneCollection, IsochroneDisplayMode},
    utils::{timetable_end_date, timetable_start_date},
};

pub async fn run_service(hrdf: Hrdf) {
    println!("Starting the server...");

    let hrdf = Arc::new(hrdf);
    let hrdf_1 = Arc::clone(&hrdf);
    let hrdf_2 = Arc::clone(&hrdf);
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    #[rustfmt::skip]
    let app = Router::new()
        .route(
            "/metadata",
            get(move || metadata(Arc::clone(&hrdf_1))),
        )
        .route(
            "/isochrones",
            get(move |params| compute_isochrones(Arc::clone(&hrdf_2), params)),
        )
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8100").await.unwrap();

    println!("Listening on 0.0.0.0:8100...");

    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Serialize)]
struct MetadataResponse {
    start_date: NaiveDate,
    end_date: NaiveDate,
}

async fn metadata(hrdf: Arc<Hrdf>) -> Json<MetadataResponse> {
    Json(MetadataResponse {
        start_date: timetable_start_date(hrdf.data_storage().timetable_metadata()),
        end_date: timetable_end_date(hrdf.data_storage().timetable_metadata()),
    })
}

#[derive(Debug, Deserialize)]
struct ComputeIsochronesRequest {
    origin_point_latitude: f64,
    origin_point_longitude: f64,
    departure_date: NaiveDate,
    departure_time: NaiveTime,
    time_limit: u32,
    isochrone_interval: u32,
    display_mode: String,
}

async fn compute_isochrones(
    hrdf: Arc<Hrdf>,
    Query(params): Query<ComputeIsochronesRequest>,
) -> Result<Json<IsochroneCollection>, StatusCode> {
    // The coordinates are not checked but should be.

    let start_date = timetable_start_date(hrdf.data_storage().timetable_metadata());
    let end_date = timetable_end_date(hrdf.data_storage().timetable_metadata());

    if params.departure_date < start_date || params.departure_date > end_date {
        // The departure date is outside the possible dates for the timetable.
        return Err(StatusCode::BAD_REQUEST);
    }

    if params.time_limit % params.isochrone_interval != 0 {
        // The result of dividing time_limit with isochrone_interval must be an integer.
        return Err(StatusCode::BAD_REQUEST);
    }

    if !["circles", "contour_line"].contains(&params.display_mode.as_str()) {
        // The display mode is incorrect.
        return Err(StatusCode::BAD_REQUEST);
    }

    let isochrones = hrdf.compute_isochrones(
        params.origin_point_latitude,
        params.origin_point_longitude,
        NaiveDateTime::new(params.departure_date, params.departure_time),
        Duration::minutes(params.time_limit.into()),
        Duration::minutes(params.isochrone_interval.into()),
        IsochroneDisplayMode::from_str(&params.display_mode).unwrap(),
        false,
    );
    Ok(Json(isochrones))
}

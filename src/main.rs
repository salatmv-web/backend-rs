use actix_web::{
    error, get, http::StatusCode, main, middleware::Logger, web, App, HttpResponse, HttpServer,
    Responder, Result,
};
use chrono::Local;
use derive_more::{Display, Error};
use lib::{
    parser::{self, Island},
    prayer::Prayer,
    utils::{convert_timestamp_to_date, convert_timestamp_to_string, days_into_year},
};
use serde::{Deserialize, Serialize};

mod lib;

#[derive(Debug, Display, Error, Clone)]
struct SalatError {
    message: String,
}

impl error::ResponseError for SalatError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.message.clone())
    }
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[derive(Deserialize, Debug)]
struct DataQuery {
    island: i16,
}

#[derive(Deserialize, Debug)]
struct IDQuery {
    id: i16,
}

#[derive(Deserialize, Debug)]
struct AtollQuery {
    id: i8,
}

#[derive(Serialize, Debug)]
struct TodayData {
    island: parser::Island,
    prayer_times: parser::PrayerTimes,
}

#[derive(Serialize, Debug)]
struct NextData {
    call: String,
    timestamp: i16,
    timestamp_str: String,
}

#[derive(Serialize, Debug)]
struct IslandData {
    island: Island,
}

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, World!"
}

#[get("/atoll")]
async fn get_atoll(
    data: web::Data<Prayer>,
    query: web::Query<AtollQuery>,
) -> Result<impl Responder, SalatError> {
    let atoll = data.get_atoll(query.id).ok_or_else(|| SalatError {
        message: "Failed to get atoll".to_string(),
    })?;

    Ok(web::Json(atoll))
}

#[get("/today")]
async fn today(
    data: web::Data<Prayer>,
    query: web::Query<DataQuery>,
) -> Result<impl Responder, SalatError> {
    let island = &data.get_island(query.island).ok_or_else(|| SalatError {
        message: "Island not found".to_owned(),
    })?;

    let prayer_today = &data.get_today(island.clone());

    Ok(web::Json(TodayData {
        island: island.clone(),
        prayer_times: prayer_today.clone().ok_or_else(|| SalatError {
            message: "Prayer for today not found.".to_owned(),
        })?,
    }))
}

#[get("/next")]
async fn next(
    data: web::Data<Prayer>,
    query: web::Query<DataQuery>,
) -> Result<impl Responder, SalatError> {
    let island = &data.get_island(query.island).ok_or_else(|| SalatError {
        message: "Island not found".to_owned(),
    })?;

    let prayer_error = SalatError {
        message: "Prayer for next not found".to_owned(),
    };

    let prayer_today = &data
        .get_today(island.clone())
        .ok_or_else(|| prayer_error.clone())?;

    let now = Local::now();

    let call = &data
        .timings
        .iter()
        .find(|p| {
            convert_timestamp_to_date(prayer_today.get_value(p.as_str()).into()).expect("Dickhead")
                >= now
        })
        .cloned();

    let new_call: String;
    let new_prayer = if call.is_none() {
        new_call = "fajr".to_owned();
        data.get_entry_from_day((days_into_year(now.date()) + 1) % 366, island.clone())
            .ok_or(prayer_error)?
    } else {
        new_call = call.as_ref().unwrap().clone();
        prayer_today.clone()
    };

    Ok(web::Json(NextData {
        call: new_call.clone(),
        timestamp: new_prayer.get_value(&new_call),
        timestamp_str: convert_timestamp_to_string(new_prayer.get_value(new_call.as_str()).into()),
    }))
}

#[get("/island")]
async fn island_get(
    data: web::Data<Prayer>,
    query: web::Query<IDQuery>,
) -> Result<impl Responder, SalatError> {
    Ok(web::Json(IslandData {
        island: data.get_island(query.id).ok_or_else(|| SalatError {
            message: "Island not found.".to_owned(),
        })?,
    }))
}

#[get("/islands")]
async fn islands_get(data: web::Data<Prayer>) -> impl Responder {
    web::Json(data.islands.clone())
}

#[main]
async fn main() -> eyre::Result<()> {
    lib::log::setup_logger()?;

    let web_data = web::Data::new(Prayer {
        atolls: parser::convert_csv("atolls".to_owned())?,
        islands: parser::convert_csv("islands".to_owned())?,
        prayers: parser::convert_csv("prayertimes".to_owned())?,
        timings: vec![
            "fajr".to_owned(),
            "sunrise".to_owned(),
            "duhr".to_owned(),
            "asr".to_owned(),
            "maghrib".to_owned(),
            "isha".to_owned(),
        ],
    });

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(today)
            .service(next)
            .service(island_get)
            .service(islands_get)
            .app_data(web_data.clone())
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 2347))?
    .run()
    .await?;

    Ok(())
}

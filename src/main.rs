use actix_web::{
    error, get, main, middleware::Logger, web, App, HttpResponse, HttpServer, Responder, Result,
};
use derive_more::{Display, Error};
use lib::{
    parser,
    prayer::{Prayer, Salat},
    utils::convert_timestamp_to_date,
};
use log::info;
use serde::{Deserialize, Serialize};

mod lib;

#[derive(Debug, Display, Error)]
struct SalatError {
    message: String,
}

impl error::ResponseError for SalatError {}

#[derive(Deserialize, Debug)]
struct DataQuery {
    island: i16,
}

#[derive(Serialize, Debug)]
struct TodayData {
    island: parser::Island,
    prayer_times: parser::PrayerTimes,
}

#[get("/")]
async fn hello() -> impl Responder {
    info!("/ 200");
    HttpResponse::Ok().body("Hello, World!")
}

#[get("/today")]
async fn today(
    data: web::Data<Prayer>,
    query: web::Query<DataQuery>,
) -> Result<impl Responder, SalatError> {
    let island = &data.get_island(query.island).ok_or(SalatError {
        message: "Island not found".to_owned(),
    })?;

    let prayer_today = &data.get_today(island.to_owned());

    let result = Ok(web::Json(TodayData {
        island: island.to_owned(),
        prayer_times: prayer_today.to_owned().ok_or(SalatError {
            message: "Prayer for today not found.".to_owned(),
        })?,
    }));

    info!("/today 200");

    result
}

#[get("/next")]
async fn next(
    data: web::Data<Prayer>,
    query: web::Query<DataQuery>,
) -> Result<impl Responder, SalatError> {
    let island = &data.get_island(query.island).ok_or(SalatError {
        message: "Island not found".to_owned(),
    })?;

    let prayer_today = &data.get_today(island.to_owned());

    Ok("")
}

#[main]
async fn main() -> Result<()> {
    lib::log::setup_logger().expect("Logger initialization failed");

    let web_data = web::Data::new(Prayer {
        atolls: parser::convert_csv("atolls"),
        islands: parser::convert_csv("islands"),
        prayers: parser::convert_csv("prayertimes"),
        timings: vec![
            "fajr".to_owned(),
            "sunrise".to_owned(),
            "dhuhr".to_owned(),
            "asr".to_owned(),
            "maghrib".to_owned(),
            "isha".to_owned(),
        ],
    });

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(today)
            .app_data(web_data.clone())
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 2347))?
    .run()
    .await?;

    Ok(())
}

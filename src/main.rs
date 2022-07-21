use actix_web::{ HttpServer, HttpResponse, Responder, App, main, get, web };
use lib::{ parser, prayer::{ Salat, Prayer } };
use log::info;
use serde::Deserialize;
use std::io::Result;

mod lib;

#[derive(Deserialize, Debug)]
struct DataQuery {
    island: i16
}


#[get("/")]
async fn hello() -> impl Responder {
    info!("/ 200");
    HttpResponse::Ok().body("Hello, World!")
}


#[get("/today")]
async fn today(data: web::Data<Prayer>, query: web::Query<DataQuery>) -> impl Responder {
    let island = &data.get_island(query.island);

    let prayer_today = &data.get_today(island.to_owned().expect("yoyo"));

    web::Json(prayer_today.to_owned().expect("Nutz"))
}



#[main]
async fn main() -> Result<()> {
    lib::log::setup_logger().expect("Failed to setup the logger.");
    

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(today)
            .app_data(web::Data::new(Prayer {
                atolls: parser::convert_csv("atolls"),
                islands: parser::convert_csv("islands"),
                prayers: parser::convert_csv("prayertimes"),
            }))
    })
    .bind(("127.0.0.1", 2347))?
    .run()
    .await?;

    Ok(())
}
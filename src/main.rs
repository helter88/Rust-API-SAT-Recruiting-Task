use actix_web::{web, HttpResponse, get, App, HttpServer};
use serde::Deserialize;
use rand::Rng;

#[derive(Deserialize)]
struct QueryParams {
    distance: u32,
    year_of_production: u32,
    fuel_usage_per_100km: u32,
}

#[get("/calculateDisselUsageForDistance")]
async fn calculate_dissel_usage(query: web::Query<QueryParams>) -> HttpResponse {
    let distance = query.distance;
    let _year_of_production = query.year_of_production;
    let fuel_usage_per_100km = query.fuel_usage_per_100km;

    let fuel_usage = (distance as f32 / 100.0) * fuel_usage_per_100km as f32;


    HttpResponse::Ok().json(fuel_usage)
}

#[derive(Deserialize)]
struct VIN {
   vin: String
}

#[get("/probabilityOfUnitInjectorFail")]
async fn probability_of_unit_injector_fail(query: web::Query<VIN>) -> HttpResponse {
    let _vin = &query.vin;
    let probability: f32 = rand::thread_rng().gen_range(0.0..1.0);

    HttpResponse::Ok().json(probability)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(calculate_dissel_usage)
            .service(probability_of_unit_injector_fail)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
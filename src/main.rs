use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};

mod schema;
mod handlers;

// Define the database connection pool
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize)]
struct ParticipationRateResponse {
    participation_rate: f64,
}

// API route to get the entire network’s participation rate
async fn get_network_participation_rate(pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let result = web::block(move || handlers::get_network_participation_rate(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        });

    match result {
        Ok(participation_rate) => HttpResponse::Ok().json(ParticipationRateResponse {
            participation_rate,
        }),
        Err(response) => response,
    }
}

// API route to get a specific validator's participation rate
async fn get_validator_participation_rate(
    pool: web::Data<Pool>,
    path: web::Path<(i32,)>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let validator_id = path.0;

    let result = web::block(move || handlers::get_validator_participation_rate(&conn, validator_id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        });

    match result {
        Ok(participation_rate) => HttpResponse::Ok().json(ParticipationRateResponse {
            participation_rate,
        }),
        Err(response) => response,
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Set up the database connection pool
    let database_url = "postgres://your_username:your_password@localhost/your_database";
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/network/participation_rate", web::get().to(get_network_participation_rate))
            .route("/validator/{id}/participation_rate", web::get().to(get_validator_participation_rate))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

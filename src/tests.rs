use actix_web::{test, App};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use std::env;

use crate::handlers::{get_network_participation_rate, get_validator_participation_rate};

fn create_test_app() -> App {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    App::new().data(pool)
}

#[actix_rt::test]
async fn test_network_participation_rate() {
    let mut app = test::init_service(create_test_app()).await;

    let req = test::TestRequest::get()
        .uri("/network/participation_rate")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let response: handlers::ParticipationRateResponse = serde_json::from_slice(&body).unwrap();

    // Assert that the participation rate is within the range [0, 1]
    assert!(response.participation_rate >= 0.0 && response.participation_rate <= 1.0);
}

#[actix_rt::test]
async fn test_validator_participation_rate() {
    let mut app = test::init_service(create_test_app()).await;

    // Replace `{validator_id}` with the ID of the validator you want to test
    let validator_id = 1;
    let req = test::TestRequest::get()
        .uri(&format!("/validator/{}/participation_rate", validator_id))
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let response: handlers::ParticipationRateResponse = serde_json::from_slice(&body).unwrap();

    // Assert that the participation rate is within the range [0, 1]
    assert!(response.participation_rate >= 0.0 && response.participation_rate <= 1.0);
}

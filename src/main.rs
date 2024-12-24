use axum::Router;
use controllers::{product_controller, stillimage_controller};
use services::ServiceRegister;
mod controllers;
mod repositories;
mod services;
mod errors;
mod dtos;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let pool = deadpool_tiberius::Manager::from_ado_string("Server=devserver;Database=Schema24-Testing;User Id=sa;Password=Chindogu1471;TrustServerCertificate=true;").unwrap()
    .max_size(20)
    .wait_timeout(0)
    .create_pool().unwrap();

    println!("Available connections: {:?}", pool.status());

    let service_register = ServiceRegister::new(pool);

    // Create individual route handlers
    let still_routes = stillimage_controller::router().with_state(service_register.stillimage_service.unwrap());
    let product_routes = product_controller::router().with_state(service_register.product_service.unwrap());

    
    let app = Router::new()
        .merge(still_routes)
        .merge(product_routes);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:81").await.unwrap();
    let _ = axum::serve(listener, app).await;
}
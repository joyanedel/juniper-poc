mod database;
mod graphql;
mod models;
mod schema;
mod server;
mod settings;

use std::{io, sync::Arc};

use actix_web::{web::Data, App, HttpServer};
use graphql::schema::create_schema;
use server::{graphql as graphql_endpoint, graphql_playground};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let schema = Arc::new(create_schema());

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql_endpoint)
            .service(graphql_playground)
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    println!("Serving on http://localhost:8080/graphql");
    println!("Playground at http://localhost:8080/playground");
    server.await
}

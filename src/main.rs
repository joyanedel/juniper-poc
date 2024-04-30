mod database;
mod graphql;
mod server;
mod settings;

use std::{io, sync::Arc};

use actix_web::{web::Data, App, HttpServer};
use graphql::schema::create_schema;
use server::{graphql as graphql_endpoint, graphql_playground};

#[actix_web::main]
async fn main() -> io::Result<()> {
    // let constants = Constants::new();
    // println!("{:#?}", constants);

    let schema = Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql_endpoint)
            .service(graphql_playground)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

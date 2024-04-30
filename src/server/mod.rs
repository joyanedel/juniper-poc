use actix_web::{get, route, web, HttpResponse, Responder};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

use crate::graphql::schema::Schema;

#[get("/playground")]
pub async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, &()).await;

    HttpResponse::Ok().json(user)
}

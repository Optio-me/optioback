use actix_web::{guard, web, http, App, HttpResponse, HttpRequest, HttpServer, Result};

//Graphql Imports
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};
use actix_cors::Cors;

//Importing required files
mod dbs;
mod gql;
mod articles;

use crate::gql::{generate_schema}; //Build Schema 


async fn index(schema: web::Data<async_graphql::Schema<gql::queries::QueryRoot, gql::mutations::Mutation, async_graphql::EmptySubscription>>, gql_req: Request, req: HttpRequest) -> Response {
    let mut gql_req = gql_req.into_inner();

    let token = req
    .headers()
    .get("Authorization")
    .and_then(|value| value.to_str().map(|s| s.to_string()).ok());

    // Auth stuff
    // if let Some(auth) = token {
    //     return match get_session(auth.clone()) {
    //         Ok(_session) => {
    //             gql_req = gql_req.data(_session);
    //             schema.execute(gql_req).await.into()
    //         },
    //         Err(_e) => {
    //             let _error = async_graphql::Response::from_errors(vec![async_graphql::ServerError::new("Not Authorized.", None)]);
    //             return Response::from(_error);
    //         }
    //     }
    // } else {
    //     if !gql_req.query.contains("authenticate")  {
    //         return Response::from(async_graphql::Response::from_errors(vec![async_graphql::ServerError::new("Not Authorized.", None)]));
    //     }
        
    // }

    schema.execute(gql_req).await.into()

}

//GraphQL playground:
async fn playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/")
        )))
}

//Actix web server:
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = generate_schema().await;
    HttpServer::new(move || {
        let cors = Cors::default()
              .allowed_origin("http://192.168.0.106:4000")
              .allowed_origin("http://192.168.0.106:5000")
              .allowed_origin("http://localhost:5000")
              .allowed_origin("http://localhost")
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE);

        App::new()
            .data(schema.clone())
            .wrap(cors)
            .service(web::resource("/")
                .guard(guard::Post()).to(index)) //Allows posting to retrieve data
            .service(web::resource("/")
                .guard(guard::Get()).to(playground)) //Gives access to playground
    })
    .bind("192.168.0.106:4000")?
    .run()
    .await
}
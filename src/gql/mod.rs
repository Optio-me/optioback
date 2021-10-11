//These are where all of our queries/mutations will reside:
pub mod mutations;
pub mod queries;
//-----------------

use async_graphql::{EmptySubscription, Schema};

use crate::gql::queries::QueryRoot;
use crate::gql::mutations::Mutation;
use crate::dbs::mongo;

pub async fn generate_schema() -> Schema<QueryRoot, Mutation, EmptySubscription> {
    let mongo_data = mongo::Connect::init().await; //Connect to database

    Schema::build(QueryRoot, Mutation, EmptySubscription)
        .data(mongo_data)
        .finish() //Build our perfect schema <3
}
use async_graphql::{Object, Error, Context};
use base64::encode;
use crate::dbs::mongo::Connect;
use crate::articles::{self, models::NewArticle};

pub struct Mutation;

#[Object]
impl Mutation {
      //Will contain all mutations
      async fn insert_article(&self, ctx: &Context<'_>, new_article: NewArticle, name: String) -> std::result::Result<String, Error> {
        articles::services::insert_article(ctx.data_unchecked::<Connect>().dbref.clone(), new_article, name).await
    }
}
use async_graphql::{Error, Context};
use crate::dbs::mongo::Connect;

use crate::articles::{self, models::Article};
pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    // Query all articles,
    async fn all_articles(
        &self,
        ctx: &Context<'_>,
    ) -> std::result::Result<Vec<Article>, async_graphql::Error> { 
        //Database instance
        let db = ctx.data_unchecked::<Connect>().dbref.clone();
        articles::services::all_articles(db).await
    }

    
}
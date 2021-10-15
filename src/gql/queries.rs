use async_graphql::{Context};
use crate::dbs::mongo::Connect;

use crate::articles::{self, models::Article};
use crate::auth::{self, services::SafeUser};
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

    async fn get_user(&self, ctx: &Context<'_>, id: String) 
    -> std::result::Result<SafeUser, async_graphql::Error> {
        let db = ctx.data_unchecked::<Connect>().dbref.clone();
        auth::services::get_user(db, id).await
    }
}
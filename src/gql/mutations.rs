use crate::articles::{self, models::NewArticle};
use crate::auth::{
    self,
    services::{get_tokens, insert_user, Session},
};
use crate::dbs::mongo::Connect;
use async_graphql::{Context, Error, Object};
use base64::encode;

pub struct Mutation;

#[Object]
impl Mutation {
    // Modifying and Inserting data into the database
    
    async fn insert_article(
        &self,
        ctx: &Context<'_>,
        new_article: NewArticle,
        name: String,
    ) -> std::result::Result<String, Error> {
        articles::services::insert_article(
            ctx.data_unchecked::<Connect>().dbref.clone(),
            new_article,
            name,
        )
        .await
    }

    async fn encode_file(&self, file_name: String) -> std::result::Result<String, Error> {
        return match std::fs::read(file_name) {
            Ok(d) => Ok(encode(&d[..])),
            Err(_) => Err(Error::new("Failed to find & encode file.")),
        };
    }

    async fn edit_name(
        &self,
        ctx: &Context<'_>,
        id: String,
        name: String,
    ) -> Result<bool, async_graphql::Error> {
        auth::services::edit_name(ctx.data_unchecked::<Connect>().dbref.clone(), id, name).await
    }

    async fn save_item(
        &self,
        ctx: &Context<'_>,
        user: String,
        id: String,
    ) -> Result<bool, async_graphql::Error> {
        auth::services::save_item(ctx.data_unchecked::<Connect>().dbref.clone(), user, id).await
    }

    async fn authenticate(
        &self,
        _ctx: &Context<'_>,
        code: String,
    ) -> std::result::Result<String, Error> {
        let tokens = get_tokens(code)?;
        return if let Ok(_r) = insert_user(
            _ctx.data_unchecked::<Connect>().dbref.clone(),
            &tokens.access_token,
            &tokens.refresh_token,
        )
        .await
        {
            Ok(tokens.access_token)
        } else {
            Err(Error::new("Failed to insert user."))
        };
    }

    async fn session(&self, _ctx: &Context<'_>) -> std::result::Result<Session, Error> {
        let session = _ctx.data_unchecked::<Session>();
        Ok(session.clone())
    }
}

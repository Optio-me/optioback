//Required imports
use crate::articles::models::NewArticle;
use async_graphql::{Error, ErrorExtensions};
use bson::oid::ObjectId;
use futures::stream::StreamExt;
use mongodb::{bson::doc, Collection, Database};

use crate::articles::models::Article; //Tag model

//This function is used to get all tags in the database
pub async fn all_articles(db: Database) -> std::result::Result<Vec<Article>, async_graphql::Error> {
    let tag_collection = db.collection("articles"); //Get tags collection

    let mut tags: Vec<Article> = vec![]; //Defines empty array
    let mut cursor = tag_collection.find(None, None).await.unwrap(); //Query all documents

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                //println!("{}", document); -> Example of how to display document
                let tag = bson::from_bson::<Article>(bson::Bson::Document(document)).unwrap();
                tags.push(tag); //Adds the tag to a list
            }

            //If there is an error with a specific tag:
            Err(error) => Err(Error::new("articleError")
                .extend_with(|_, e| e.set("details", format!("Unable to retrieve: {}", error))))
            .unwrap(),
        }
    }

    //Length validation:

    if tags.len() > 0 {
        Ok(tags)
    } else {
        Err(Error::new("articleError").extend_with(|_, e| e.set("details", "No documents")))
    }
}

pub async fn get_article(db: Database, id: String) -> std::result::Result<Article, async_graphql::Error> {
    let articles = db.collection("articles");

    let cursor = articles.find_one(doc! {"_id": ObjectId::with_string(&id).unwrap() ,}, None).await?
        .expect("Missing document.");

    Ok(bson::from_document::<Article>(cursor).unwrap())
}


pub async fn insert_article(
    db: Database,
    mut new_article: NewArticle,
    name: String,
) -> std::result::Result<String, async_graphql::Error> {
    let article_collection = db.collection("articles");

    new_article.name = name;
    let new_article_bson = bson::to_bson(&new_article).unwrap();

    if let bson::Bson::Document(document) = new_article_bson {
        // Insert into a MongoDB collection
        return match article_collection.insert_one(document, None)
            .await
            {
                Ok(_d) => {
                    let oid = _d.inserted_id.as_object_id().unwrap().clone();
                    Ok(oid.to_hex())
                }
                Err(_) => Err(async_graphql::Error::new("Failed.")),
            };
    } else {
        Err(Error::new("new-article").extend_with(|_, e| {
            e.set("details", "Error inserting article!")
        }))
    }
}

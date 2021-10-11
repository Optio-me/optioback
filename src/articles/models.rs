use serde::{Serialize, Deserialize};
use bson::oid::ObjectId; //Required for the ObjectId type

use async_graphql::{InputObject, Json};

#[derive(Serialize, Deserialize, Clone)]
pub struct Article {
    pub _id: ObjectId,
    pub name: String
}

#[async_graphql::Object]
impl Article {
    pub async fn _id(&self) -> String {
        //We must convert the ID to a hex representation otherwise a map is returned.
        self._id.to_hex()
    }

    pub async fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Serialize, Deserialize, Clone, InputObject)]
pub struct NewArticle {
    pub name: String
}

#[allow(dead_code)]
impl NewArticle {
    pub async fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
#![allow(non_snake_case)]
use async_graphql::SimpleObject;
use bson::doc;
use mongodb::options::UpdateOptions;
use mongodb::Database;
use reqwest::header::{AUTHORIZATION, ORIGIN};
use bson::oid::ObjectId;

#[derive(serde::Deserialize, Clone)]
#[allow(dead_code)]
pub struct TokenResp {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub scope: String,
    pub refresh_token: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, SimpleObject)]
pub struct Session {
    pub displayName: String,
    pub givenName: String,
    pub id: String,
    pub userPrincipalName: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct User {
    info: Session,
    admin: bool,
    refresh_token: String,
    saved : Vec<ObjectId>
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SafeUser {
    info: Session,
    admin: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    saved: Vec<ObjectId>
}

#[async_graphql::Object]
impl SafeUser {
    async fn info(&self) -> Session {
        self.info.clone()
    }

    async fn admin(&self) -> bool {
        self.admin.clone()
    }
    async fn saved(&self) -> Vec<String> {
        self.saved.iter()
        .map(|i| i.to_hex()).collect()
    }
}

#[tokio::main]
pub async fn get_tokens(code: String) -> Result<TokenResp, async_graphql::Error> {
    let client = reqwest::Client::new();
    let params = [
        ("client_id", "6b45745d-2213-4238-a435-9e8e06e0b974"),
        ("scope", "https://graph.microsoft.com/user.read"),
        ("code", &code),
        ("redirect_uri", "http://localhost:5000/"),
        ("grant_type", "authorization_code"),
        ("code_verifier", "rsn"),
    ];

    let res = client
        .post("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .form(&params)
        .header(ORIGIN, "http://localhost")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    if res["error"] == "invalid_grant" {
        Err(async_graphql::Error::new("Invalid code."))
    } else {
        let tokens: TokenResp = serde_json::from_value(res).unwrap();
        Ok(tokens)
    }
}

#[tokio::main]
pub async fn get_session(access_token: String) -> Result<Session, async_graphql::Error> {
    let client = reqwest::Client::new();

    let res = client
        .get("https://graph.microsoft.com/v1.0/me")
        .header(AUTHORIZATION, access_token)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;


    let session: Result<Session, serde_json::Error> = serde_json::from_value(res);
    
    return match session {
        Ok(s) => Ok(s),
        Err(_e) => Err(async_graphql::Error::new("Invalid.")),
    };
}

pub async fn get_user(db: Database, id: String) -> std::result::Result<SafeUser, async_graphql::Error> {
    let images = db.collection("users");

    //TODO: Add session check (only user can request their own profile)

    let cursor = images.find_one(doc! {"info.id": id}, None).await?
        .expect("Missing document.");

    Ok(bson::from_document::<SafeUser>(cursor).unwrap())
}

pub async fn save_item(
    db: Database,
    user: String,
    id: String
) -> Result<bool, async_graphql::Error> {
    let collection = db.collection("users");

    return match collection
        .update_one(
            doc! {"info.id": user},
            doc! {
             "$push": { "saved": ObjectId::with_string(&id).unwrap() }
            },
            None,
        )
        .await
    {
        Ok(_res) => Ok(true),
        Err(_e) => Err(async_graphql::Error::new("Failed to add saved item.")),
    };
}

pub async fn edit_name(
    db: Database,
    id: String,
    name: String,
) -> Result<bool, async_graphql::Error> {
    let collection = db.collection("users");

    //TODO: Add session check (only user can edit their own name) - Use refresh token?

    return match collection
        .update_one(
            doc! {"info.id": id},
            doc! {
               "$set": { "info.displayName": name }
            },
            None,
        )
        .await
    {
        Ok(_res) => Ok(true),
        Err(_e) => Err(async_graphql::Error::new("Failed to edit name.")),
    };
}

pub async fn insert_user(
    db: Database,
    access_token: &str,
    refresh_token: &str,
) -> Result<bool, async_graphql::Error> {
    let collection = db.collection("users");

    if let Ok(s) = get_session(access_token.to_string()) {
        let document = User {
            info: s.clone(),
            refresh_token: refresh_token.to_string(),
            admin: false,
            saved: Vec::new()
        };

        let query = doc! {"info.id" : s.id.clone()};
        let update = bson::to_document(&document).unwrap();
        let options = UpdateOptions::builder().upsert(true).build();

        return match collection.update_one(query, update, options).await {
            Ok(_res) => Ok(true),
            Err(_e) => Err(async_graphql::Error::new("Failed to create user.")),
        };
    } else {
        Err(async_graphql::Error::new(
            "Failed to get session in user creation.",
        ))
    }
}

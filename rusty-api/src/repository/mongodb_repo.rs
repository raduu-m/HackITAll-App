use std::env;
extern crate dotenv;
use actix_web::Error;
use dotenv::dotenv;

use mongodb::{
    bson::{doc, oid::ObjectId, Bson},
    options::{ClientOptions, FindOneOptions, FindOptions, UpdateOptions, Collation},
    Client,Collection, results::{InsertOneResult,UpdateResult}
};

use crate::models::user_model::User;

pub struct MongoDBRepo {
    col: Collection<User>,
}

impl MongoDBRepo {
    pub async fn init() -> Self{
        dotenv().ok();
        let mongo_uri = match env::var("MONGO_URI") {
            Ok(val) => val,
            Err(e) => panic!("MONGO_URI not found in .env file"),
        };
        let client = Client::with_uri_str(&mongo_uri).await.unwrap();
        let db = client.database("testDB");
        let col: Collection<User> = db.collection("users");
        MongoDBRepo { col }
    }

    pub async fn create_user(&self, new_user:User) -> Result<InsertOneResult,Error>{
        let new_doc = User {
            id: None,
            name: new_user.name,
            email: new_user.email,
            password: new_user.password,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub async fn update_user(&self, id:&String, new_user:User)->Result<UpdateResult, Error>{
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {"$set": {
            "name": new_user.name,
            "email": new_user.email,
            "password": new_user.password,
        }};
        let update_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(update_doc)
    }
}
use std::env;
extern crate dotenv;
use actix_web::Error;
use dotenv::dotenv;

use mongodb::{
    bson::{doc, oid::ObjectId, Bson},
    options::{ClientOptions, FindOneOptions, FindOptions, UpdateOptions, Collation},
    Client,Collection, results::{InsertOneResult,UpdateResult, DeleteResult}
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
        let db = client.database("db-hack");
        let col: Collection<User> = db.collection("users");
        MongoDBRepo { col }
    }

    pub async fn get_user(&self, user:User) -> Result<Option<User>,Error>{
        let new_doc = User{
            id: user.id,
            name: user.name,
            email: user.email,
            password: user.password,
            balance: user.balance,
        };
        let filter = doc! {"id":new_doc.id ,"email": new_doc.email, "password": new_doc.password};
        let user = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error finding user");
        Ok(user)
    }

    pub async fn create_user(&self, new_user:User) -> Result<InsertOneResult,Error>{
        let new_doc = User {
            id: new_user.id,
            name: new_user.name,
            email: new_user.email,
            password: new_user.password,
            balance: 0.0,
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
        let obj_id = id;
        let filter = doc! {"id": obj_id};
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

    pub async fn delete_user(&self, id:&String)->Result<DeleteResult, Error>{
        let obj_id = id;
        let filter = doc! {"id": obj_id};
        let update_doc = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");
        Ok(update_doc)
    }
}
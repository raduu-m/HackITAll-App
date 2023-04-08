mod api;
mod models;
mod repository;

use actix_web::{get,post,put,delete,App, HttpResponse, HttpServer, Responder, web::{Data, Json, Path}};
use repository::mongodb_repo::MongoDBRepo;
use mongodb::bson::oid::ObjectId;
use models::user_model::User;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[post("/user/login/{id}")]
async fn login_user(db: Data<MongoDBRepo>,path:Path<String> ,user: Json<User>) -> impl Responder{
    let id = path.into_inner();
    let data = User{
        id: id,
        name: user.name.to_owned(),
        email: user.email.to_owned(),
        password: hashPassword(user.password.to_owned()),
    };
    let user_details = db.get_user(data).await;
    match user_details {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

fn hashPassword(password:String) -> String{
    // Encrypt use the Cesar Cipher
    let mut encrypted_password = String::new();
    for c in password.chars() {
        encrypted_password.push((c as u8 + 3) as char);
    }
    encrypted_password
}

fn decryptPassword(password:String)->String{
    // Decrypt use the Cesar Cipher
    let mut decrypted_password = String::new();
    for c in password.chars() {
        decrypted_password.push((c as u8 - 3) as char);
    }
    decrypted_password
}

#[post("/user")]
async fn create_user(db: Data<MongoDBRepo>, new_user: Json<User>) -> impl Responder {
    let data = User{
        id: new_user.id.to_owned(),
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: hashPassword(new_user.password.to_owned()),
    };
    let user_details = db.create_user(data).await;
    match user_details {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[put("/user/{id}")]
pub async fn update_user(db:Data<MongoDBRepo>,path:Path<String>,new_user:Json<User>)->impl Responder{
    let id = path.into_inner();
    let data = User{
        id: new_user.id.to_owned(),
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: hashPassword(new_user.password.to_owned()),
    };
    let user_details = db.update_user(&id,data).await;
    match user_details {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[delete("/user/{id}")]
pub async fn delete_user(db:Data<MongoDBRepo>,path:Path<String>) -> impl Responder{
    let id = path.into_inner();
    let user_details = db.delete_user(&id).await;
    match user_details {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoDBRepo::init().await;
    let db_date = Data::new(db);
    HttpServer::new(move || App::new().app_data(db_date.clone()).service(hello).service(create_user).service(delete_user).service(update_user).service(login_user))
        .bind(("localhost", 8080))?
        .run()
        .await
}
mod api;
mod models;
mod repository;

use actix_web::{get,post,put,App, HttpResponse, HttpServer, Responder, web::{Data, Json, Path}};
use repository::mongodb_repo::MongoDBRepo;
use mongodb::bson::oid::ObjectId;
use models::user_model::User;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[post("/user")]
async fn create_user(db: Data<MongoDBRepo>, new_user: Json<User>) -> impl Responder {
    let data = User{
        id: None,
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
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
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
    };
    let user_details = db.update_user(&id,data).await;
    match user_details {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[post("/test")]
async fn test() -> impl Responder {
    HttpResponse::Ok().json("test")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoDBRepo::init().await;
    let db_date = Data::new(db);
    HttpServer::new(move || App::new().app_data(db_date.clone()).service(hello).service(create_user).service(test).service(update_user))
        .bind(("localhost", 8080))?
        .run()
        .await
}
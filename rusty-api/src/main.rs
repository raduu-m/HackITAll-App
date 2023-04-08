mod api;
mod models;
mod repository;

use actix_web::{get,post,put,delete,App, HttpResponse, HttpServer, Responder, web::{Data, Json, Path}, Error};
use repository::mongodb_repo::MongoDBRepo;
use mongodb::bson::oid::ObjectId;
use models::user_model::User;
use models::transaction_model::Transaction;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[post("/user/login/{email}/{password}")]
async fn login_user(db: Data<MongoDBRepo>,path:Path<(String,String)>) -> impl Responder{
    // Get the email and password from the path
    let (email,password) = path.into_inner();
    println!("Email : {}",email);
    println!("Password :{}",password);
    let data = User{
        id: "none".to_owned(),
        name: "none".to_owned(),
        email: email.to_owned(),
        password: hashPassword(password.to_owned()),
        balance: 0.0,
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
        balance: 0.0,
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
        balance: new_user.balance,
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

#[get("/user/{id}")]
async fn sync_user(db: Data<MongoDBRepo>,path:Path<(String)>) -> impl Responder{
    // Get the email and password from the path
    let (uid) = path.into_inner();
    println!("user id : {}",uid);

    let user_details = db.get_user_from_id(uid).await;
    match user_details {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}


#[get("/user/transactions/{id}")]
async fn sync_transactions_for_id(db: Data<MongoDBRepo>,path:Path<(String)>) -> impl Responder{
    // Get the email and password from the path
    let (uid) = path.into_inner();
    println!("user id : {}",uid);

    //let mut transactions Vec::new();

    let res_received = db.get_received_transaction(uid.clone()).await;
    let res_sent = db.get_sent_transaction(uid.clone()).await;

    println!("R{:?}",res_received);
    println!("S{:?}",res_sent);


    match res_received {
        Ok(t_received) => match res_sent{
            Ok(t_sent) => {
                let transactions: Vec<Transaction> = t_received.into_iter().chain(t_sent.into_iter()).collect();
                // println!("T_SENT {:?}",transactions);
                
                HttpResponse::Ok().json(transactions)
            },
            Err(e) => HttpResponse::InternalServerError().json(e.to_string())
        },
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}


#[post("/transaction")]
async fn create_transaciton(db: Data<MongoDBRepo>, new_transaction: Json<Transaction>) -> impl Responder {
    let data = Transaction{
        id: None,
        timestamp: new_transaction.timestamp.to_owned(),
        t1_id: new_transaction.t1_id.to_owned(),
        t2_id: new_transaction.t2_id.to_owned(),
        ammount: new_transaction.ammount.to_owned()
    };
    let id_clone = data.t1_id.clone();
   
    let user_details = db.get_user_from_id(id_clone).await;
    let user_receiving_details = db.get_user_from_id(data.t2_id.clone()).await;


    match user_receiving_details{
        Ok(user_r) => 
            match user_r{
                Some(u_r) => { 
                    match user_details{
                        Ok(user) => {
                            match user{
                                Some(u) => 
                                    if(u.balance - data.ammount >= 0.0){
                                        let res = db.update_user_balance(&u.id,u.balance - data.ammount ).await;
                                        let res_receiving = db.update_user_balance(&u_r.id, u_r.balance + data.ammount).await;
                                        match res_receiving{
                                            Ok(_update) => 

                                            match res{
                                                Ok(_u) => {
                                                    let transaction_details = db.create_transaciton(data).await;
                                                
                                                    match transaction_details {
                                                        Ok(transaction) => return HttpResponse::Ok().json(transaction),
                                                        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
                                                    }},
                                                Err(e)  => return HttpResponse::InternalServerError().json(e.to_string())
                                            }

                                            ,
                                            Err(e) => return HttpResponse::InternalServerError().json(e.to_string())
                                        }

                                    }else{
                                        return HttpResponse::InternalServerError().json("insufficient funds".to_string())
                                    }
                                None => return HttpResponse::InternalServerError().json("no user to transfer from ".to_string())    
                            }
                    
                        },
                        Err(e) => return HttpResponse::InternalServerError().json(e.to_string())
                        
                    }
                },
                None => HttpResponse::InternalServerError().json("no user to transfer to ".to_string())  
            }

        Err(e) => HttpResponse::InternalServerError().json(e.to_string())
    }

}

#[put("/transactionx/")]
pub async fn revert_transaction(db:Data<MongoDBRepo>,bad_transaction:Json<Transaction>)->impl Responder{
    let data = Transaction{
        id: None,
        timestamp: bad_transaction.timestamp.to_owned(),
        t1_id: bad_transaction.t1_id.to_owned(),
        t2_id: bad_transaction.t2_id.to_owned(),
        ammount: bad_transaction.ammount.to_owned()
    };
    let transaction_details = db.revert_transaction(data).await;
    match transaction_details {
        Ok(transaction) => HttpResponse::Ok().json(transaction),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoDBRepo::init().await;
    let db_date = Data::new(db);

    HttpServer::new(move || App::new().app_data(db_date.clone()).service(hello).service(create_user).service(delete_user).service(update_user).service(login_user)
    .service(revert_transaction).service(create_transaciton).service(sync_user).service(sync_transactions_for_id))
        .bind(("localhost", 8080))?
        .run()
        .await
}
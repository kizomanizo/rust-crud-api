// @Author Kizito kizomanizo@gmail.com

// Project modules
mod api;
mod models;
mod repository;

// My imports for this project
use actix_web::{web::Data, App, HttpServer};
use api::user::{create_user, get_user, update_user, delete_user, get_all_users};
use repository::mongodb::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            .service(get_all_users)
    })
    .bind(("127.0.0.1", 8082))?
    .run()
    .await
}
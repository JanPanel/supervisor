use actix_web::{
    middleware::Logger,
    post,
    web::{Json, JsonConfig},
    App, HttpServer, Responder,
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dotenv::dotenv;
use supervisor::models::User;
use std::{env, error::Error};
use uuid::Uuid;

type DbPool = Pool<ConnectionManager<PgConnection>>;


#[post("/users")]
async fn create_user(data: Json<User>) -> impl Responder {
    // TODO: Check if user already exists
    let user = User {
        id: Uuid::new_v4(),
        email: data.0.email,
        password: data.0.password,
        permissions: data.0.permissions,
    };

    // TODO: Save user to db
    Json(user)
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let database_url = env::var("DATABASE_URL")?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?;

    let bind_url = format!("{}:{}", env::var("HOST")?, env::var("PORT")?);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(JsonConfig::default().limit(4096))
            .data(pool.clone())
            .service(create_user)
    })
    .bind(&bind_url)?
    .run()
    .await?;

    Ok(())
}

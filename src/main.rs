use actix_web::{
    get,
    middleware::Logger,
    post,
    web::{Data, Json, JsonConfig, Path},
    App, HttpServer, Responder,
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dotenv::dotenv;
use std::{env, error::Error};
use supervisor::{models::User, schema::users};
use uuid::Uuid;

type DbPool = Pool<ConnectionManager<PgConnection>>;

// TODO: Move all routes into modules and routers

// NOTE: This service is only for testing purposes
#[get("/users")]
async fn get_users(pool: Data<DbPool>) -> impl Responder {
    let conn = pool.get().unwrap();
    let users = users::table.load::<User>(&conn).unwrap();

    Json(users)
}

#[get("/users/{id}")]
async fn get_user(pool: Data<DbPool>, Path(id): Path<Uuid>) -> impl Responder {
    let conn = pool.get().unwrap();
    let user = users::table
        .filter(users::id.eq(id))
        .load::<User>(&conn)
        .unwrap();

    // TODO: Return code 404 when user not found
    Json(user)
}

#[post("/users")]
async fn create_user(pool: Data<DbPool>, data: Json<User>) -> impl Responder {
    let conn = pool.get().unwrap();
    // TODO: Check if user already exists

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(data.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let user = User {
        id: Uuid::new_v4(),
        email: data.0.email,
        password: password_hash,
        permissions: data.0.permissions,
    };

    diesel::insert_into(users::table)
        .values(&user)
        .execute(&conn)
        .unwrap();

    Json(user)
}

// TODO: A way to obtain an access token

// TODO: Protected PATCH /users endpoint

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
            .service(get_user)
            .service(get_users)
            .service(create_user)
    })
    .bind(&bind_url)?
    .run()
    .await?;

    Ok(())
}

use actix_web::{
    middleware::Logger,
    post,
    web::{Json, JsonConfig},
    App, HttpServer, Responder,
};
use supervisor::models::User;
use uuid::Uuid;



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
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .data(JsonConfig::default().limit(4096))
            .service(create_user)
    })
        .bind("localhost:8080")?
        // TODO: Extract server address to .env
        .run()
        .await
}

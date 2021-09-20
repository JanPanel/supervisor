use actix_web::{post, web::{Json, JsonConfig}, HttpServer, App, middleware::Logger, Responder};
use supervisor::{User, NewUser};


#[post("/users")]
async fn create_user(data: Json<NewUser>) -> impl Responder {
    println!("{:?}", data.0);
    // TODO: Check if user already exists
    let user = User::new(data.0.email, data.0.password, data.0.permissions);
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

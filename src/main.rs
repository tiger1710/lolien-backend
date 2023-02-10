use actix_web::{middleware, web::Data, App, HttpServer};
use replay::ReplayContainer;
use sea_orm::Database;
use user::UserContainer;
pub mod replay;
pub mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    // set up database connection pool
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let db = Database::connect(conn_spec)
        .await
        .expect("Can't connect to db.");

    let user_container = UserContainer::new_user_container(db.clone());
    let replay_container = ReplayContainer::new_replay_container(db.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(user_container.clone()))
            .app_data(Data::new(replay_container.clone()))
            .wrap(middleware::Logger::default())
            .configure(user::config)
            .configure(replay::config)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

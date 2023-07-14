use actix_web::{HttpServer, web, App, middleware::Logger};
use file_share_server::file_operate;
use file_share_server::common_operate;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let address = common_operate::get_address();
    log::info!("Starting HTTP server at http://{}:{}", address.ip, address.port);

    HttpServer::new(|| {
        App::new()
        .configure(app_config)
        .wrap(Logger::default())
    })
    .bind((address.ip, address.port))?
    .run()
    .await
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
        .service(file_operate::file_list)
        .service(file_operate::save_files)
        .service(actix_files::Files::new("/file", "./file").show_files_listing())
        .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    );
}
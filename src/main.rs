use::std::fs;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpServer};
use chrono::{Datelike, Local};

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化项目 
    // 创建日记文件夹
    let local = Local::now();
    let year = local.year();
    let month = local.month();
    
    fs::create_dir_all(format!("./diarys/{}/{}", year, month))
        .expect("Failed to create 'diarys' folder");

    // 启用服务器
    HttpServer::new(|| {
        // CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();
        App::new()
            .wrap(cors)
            .service(api::diary::save_diary)
            .service(Files::new("/", "./web").index_file("index.html"))
    })
    .bind(("127.0.0.1", 3737))?
    .run()
    .await
}
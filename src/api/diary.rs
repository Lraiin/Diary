use::std::fs::File;
use std::{fs, io::Write};
use chrono::{Datelike, Local};
use actix_web::{post, HttpResponse, Responder};

#[post("/diary/save")]
async fn save_diary(req_body: String) -> impl Responder {
    let year = Local::now().year();
    let month = Local::now().month();
    let day = Local::now().day();
    let file_name = format!("{}-{}-{}.md", year, month, day);
    let file_path = format!("./diarys/{}/{}/", year, month);

    match fs::metadata(file_path.clone() + &file_name) {
        Ok(_) => {

        },
        Err(_) => {

        }
    }

    match File::create(file_path + &file_name) {
        Ok(mut diary) => {
            match diary.write_all(req_body.as_bytes()) {
                Ok(_) => {
                    HttpResponse::Ok().body("保存日记成功!")
                },
                Err(_) => {
                    HttpResponse::InternalServerError().body("保存日记失败! 'match diary.write_all' ERROR")
                }
            }
        },
        Err(_) => {
            return HttpResponse::InternalServerError().body("无法创建日记文件! 'match File::creat' ERROR");
        },
    }
}
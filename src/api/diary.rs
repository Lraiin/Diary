use::std::fs::File;
use std::{fs, io::Write};
use chrono::{Datelike, Local};
use actix_web::{get, post, HttpResponse, Responder};


// 保存日记
#[post("/diary/save")]
async fn save_diary(req_body: String) -> impl Responder {
    let year = Local::now().year();
    let month = Local::now().month();
    let day = Local::now().day();
    let mut counter = 1;
    let file_path = format!("./diarys/{}/{}/{}/", year, month, day);
    let file_name = format!("{}-{}-{}", year, month, day);

    loop {
        let file = format!("{}{}-{}.md", file_path, file_name, counter);
        if fs::metadata(file).is_ok() {
            counter += 1;
        } else {
            match File::create(format!("{}{}-{}.md", file_path, file_name, counter)) {
                Ok(mut diary) => {
                    match diary.write_all(req_body.as_bytes()) {
                        Ok(_) => {
                            return HttpResponse::Ok().body("保存日记成功!");
                        },
                        Err(_) => {
                            return HttpResponse::InternalServerError().body("保存日记失败! 'match diary.write_all' ERROR");
                        }
                    }
                },
                Err(_) => {
                    return HttpResponse::InternalServerError().body("无法创建日记文件夹! 'match File::creat' ERROR");
                }
            }
        }
    }

}

// 获取日记列表
#[get("/diary/list")]
async fn search_diary_list() -> impl Responder {
    HttpResponse::Ok().body("获取日记列表成功!")
}
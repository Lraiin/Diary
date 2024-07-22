use::std::fs::File;
use std::{fs, io::Write};
use chrono::{Datelike, Local};
use actix_web::{get, post, web, HttpResponse, Responder};

use serde::Serialize;


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


// 获取当天日记
#[derive(Serialize)]
struct DiaryInfo {
    diary_name: String,
    diary_path: String,
}

#[get("/diary/list/{year}/{month}/{day}")]
async fn search_diary_list(path: web::Path<(String, String, String)>) -> impl Responder {
    let (s_year, s_month, s_day) = path.into_inner();

    // 遍历目录
    let mut file_list: Vec<DiaryInfo> = Vec::new();
    if let Ok(diary_dir) = fs::read_dir(format!("./diarys/{}/{}/{}", s_year, s_month, s_day)) {
        for entry in diary_dir {
            if let Ok(entry) = entry {
                if let Some(filename) = entry.file_name().into_string().ok() {
                    let file_path = entry.path().display().to_string();
                    file_list.push( DiaryInfo {
                        diary_name: filename,
                        diary_path: file_path,
                    });
                }
            }
        }
    }
    HttpResponse::Ok().json(file_list)
}
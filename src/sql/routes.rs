#![allow(unused_imports)]

use crate::sql::models::{GroupID, UserTalk};
use crate::sql::models::Info;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, HttpRequest,Result};
use sqlx::mysql::MySqlPool;

#[get("/group_id")]
async fn find_all_group_id(db_pool: web::Data<MySqlPool>) -> impl Responder {
    let result = GroupID::find_all_group_id(db_pool.get_ref()).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        _ => HttpResponse::BadRequest().body("Error trying to read all todos from database")
    }
}

#[get("/user/{user_id}/{group_id}")]
async fn find_group_talk(req: HttpRequest, db_pool: web::Data<MySqlPool>) -> impl Responder {
    let v1 = req.match_info().query("user_id").parse::<String>().unwrap_or_default();
    let v2 = req.match_info().query("group_id").parse::<String>().unwrap_or_default();
    let result = UserTalk::find_all_talk(
        v1, v2, db_pool.get_ref(),
    ).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        _ => HttpResponse::BadRequest().body("Error trying to read all todos from database")
    }
}


#[post("/find")]
async fn find_all_talk(info: web::Json<Info>, db_pool: web::Data<MySqlPool>) -> impl Responder {
    let v1 = &info.user_id;
    let v2 = &info.group_id;
    println!("{}{}", v1, v2);
    let result = UserTalk::find_all_talk(
        v1.to_string(), v2.to_string(), db_pool.get_ref(),
    ).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        _ => HttpResponse::BadRequest().body("Error trying to read all todos from database")
    }
}



pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all_group_id);
    cfg.service(find_all_talk);
    cfg.service(find_group_talk);
}
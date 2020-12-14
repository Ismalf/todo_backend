use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{dev, error, http, web, Error, HttpResponse, Result, Responder, get, put, post, delete, options};
use serde::{Serialize, Deserialize};
use crate::models::task::{Task, NewTask};
use actix_web::web::Json;
use crate::repository::daos::{taskDAO, DAO};
use crate::database_engine_pool::{MysqlPool, get_conn};
use std::ops::Deref;
use serde_json::value::Serializer;
use actix::Response;

#[get("/allTodos")]
pub async fn get_all(pool: web::Data<MysqlPool>) -> Result<HttpResponse, Error> {
    println!("Hello World");
    // Use DAO all
    let x = web::block(move || taskDAO::get_all(get_conn(&pool).as_deref().unwrap())).await;
    let jsonRes = serde_json::to_string(&x.unwrap());
    Ok(HttpResponse::Ok().json(jsonRes.unwrap()))
}

/*#[get("/getTodo")]
pub async fn get(params: serde_json::Value, pool: web::Data<MysqlPool>) -> impl Responder {
    // Use params attributes to filter DB

    format!("Hello {}! id:{}", 0, 0)
}*/
#[options("/*")]
pub async fn ok_task() -> HttpResponse {
    HttpResponse::Ok().status(http::StatusCode::OK).set_header("Access-Control-Allow-Headers","*").body("Ok")
}

#[post("/createTodo")]
pub async fn create_task(data: Json<NewTask>, pool: web::Data<MysqlPool>) -> HttpResponse {
    println!("{}", data.name);
    // Use DAO create
    let x = web::block(move || taskDAO::save_item(data.into_inner(), get_conn(&pool).as_deref().unwrap())).await;
    let jsonRes = serde_json::to_string(&x.unwrap());
    HttpResponse::Ok().status(http::StatusCode::OK).json(jsonRes.unwrap())
}

#[put("/updateTodo")]
pub async fn update_task(data: Json<Task>, pool: web::Data<MysqlPool>) -> Result<HttpResponse, Error> {
    println!("{}", data.name);
    // Use DAO update
    let x = web::block(move || taskDAO::update_item(data.into_inner(), get_conn(&pool).as_deref().unwrap())).await;
    let jsonRes = serde_json::to_string(&x.unwrap());
    Ok(HttpResponse::Ok().json(jsonRes.unwrap()))
}

#[delete("/deleteTodo/{id}")]
pub async fn delete_task(id: web::Path<i32>, pool: web::Data<MysqlPool>) -> Result<HttpResponse, Error> {
    println!("{}", id);
    // Use DAO delete operation
    let x = web::block(move || taskDAO::delete_item(id.into_inner(), get_conn(&pool).as_deref().unwrap())).await;
    let jsonRes = serde_json::to_string(&x.unwrap());
    Ok(HttpResponse::Ok().json(jsonRes.unwrap()))
}

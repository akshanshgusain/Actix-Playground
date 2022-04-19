use std::io::ErrorKind::Other;
use crate::{db, Status};
use crate::models::{CreateTodoList, ResultResponse};
use actix_web::{HttpResponse, Responder, web};
use deadpool_postgres::{Pool, Client};


// impl Keyword is used in return position to say "the type returned will implement this trait
pub async fn status() -> impl Responder {
    HttpResponse::Ok()
        .json(Status { status: "Ok".to_string() })
}


/// Extractors

/* Application State Extractor:
   Application state is accessible from the handler with the web::Data extractor; however, state
   is accessible as a read-only reference. If you need mutable access to state, it must be implemented.
 */
pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client =
        db_pool.get().await.expect("Error connecting to the database");

    let result = db::get_todos(&client).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}


/* Alternative way to get path parameters:
#
[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().query("user_id").parse().unwrap();

    Ok(format!("Welcome {}, user_id {}!", name, userid))
}
*/
pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32, String)>) -> impl Responder {
    let client: Client =
        db_pool.get().await.expect("Error connecting to the database");
    let (list_id, _) = path.into_inner();
    let result = db::get_items(&client, list_id).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

// Get values from body as json
/*
Json<T> allows deserialization of a request body into a struct. To extract typed information from
a request’s body, the type T must implement serde::Deserialize.
 */
pub async fn create_todo(db_pool: web::Data<Pool>, json: web::Json<CreateTodoList>) -> impl Responder {
    let client: Client =
        db_pool.get().await.expect("Error connecting to the database");
    let result = db::create_todo(&client, json.title.clone()).await;

    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn check_item(db_pool: web::Data<Pool>, path: web::Path<(i32, i32)>) -> impl Responder {
    let client: Client =
        db_pool.get().await.expect("Error connecting to the database");
    let (list_id, item_id) = path.into_inner();
    let result = db::check_item(&client, list_id, item_id).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(ResultResponse { success: true }),
        Err(ref e) if e.kind() == Other => HttpResponse::Ok().json(ResultResponse { success: false }),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}













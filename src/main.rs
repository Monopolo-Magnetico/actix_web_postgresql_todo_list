use actix_web::{HttpResponse, HttpServer, Responder, delete, get, post, put, web::{self, Json}, App};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let pool= db().await;

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(todo_list)
        .service(add_todo)
        .service(get_single_todo)
        .service(delete_single_todo)
        .service(update_single_todo)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/todo_list")]
async fn todo_list(pool: web::Data<PgPool>) -> impl Responder {
    
    let todos: Vec<TodoSql> = sqlx::query_as("SELECT * FROM todos").fetch_all(pool.get_ref()).await.unwrap();

    let todos_json = serde_json::to_string_pretty(&todos).unwrap();

    HttpResponse::Ok().body(todos_json)

}

#[post("/todo_list")]
async fn add_todo(pool: web::Data<PgPool>, todo_req: Json<TodoReq>) -> impl Responder {

    sqlx::query("INSERT INTO todos (content) VALUES ($1)").bind(&todo_req.content).execute(pool.get_ref()).await.unwrap();

    HttpResponse::Created().body("Added new Todo")

}

#[get("/todo/{id}")]
async fn get_single_todo(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {

    let id = path.into_inner();

    let rows: Vec<TodoSql> = sqlx::query_as("SELECT * FROM  todos WHERE  id = $1").bind(&id).fetch_all(pool.get_ref()).await.unwrap();

    if rows.len() == 0 {
        let msg = format!("Todo with id {} not found", id);
        HttpResponse::NotFound().body(msg)
    } else {
        let todo_json = serde_json::to_string(&rows[0]).unwrap();
        HttpResponse::Ok().body(todo_json)
    }

}

#[delete("/delete/{id}")]
async fn delete_single_todo(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {

    let id = path.into_inner();

    let rows: Vec<TodoSql> = sqlx::query_as("SELECT * FROM  todos WHERE  id = $1").bind(&id).fetch_all(pool.get_ref()).await.unwrap();

    if rows.len() == 0 {
        let msg = format!("Todo with id {} not found", id);
        HttpResponse::NotFound().body(msg)
    } else {
        sqlx::query("DELETE FROM todos WHERE id = $1").bind(&id).execute(pool.get_ref()).await.unwrap();
        HttpResponse::Ok().body("Removed Todo") // Alternativo a un JSON
    }

}

#[put("/update/{id}")]
async fn update_single_todo(pool: web::Data<PgPool>, path: web::Path<i32>, todo_req: Json<TodoReq>) -> impl Responder {

    let id = path.into_inner();

    let rows: Vec<TodoSql> = sqlx::query_as("SELECT * FROM  todos WHERE  id = $1").bind(&id).fetch_all(pool.get_ref()).await.unwrap();

    if rows.len() == 0 {
        let msg = format!("Todo with id {} not found", id);
        HttpResponse::NotFound().body(msg)
    } else {
        sqlx::query("UPDATE todos SET content = $1 WHERE id = $2").bind(&todo_req.content).bind(&id).execute(pool.get_ref()).await.unwrap();
        HttpResponse::Ok().body("Updated Todo") // Alternativo a un JSON
    }

}

async fn db() -> PgPool {
    let pool = sqlx::postgres::PgPool::connect("postgres://postgres:1661!@localhost:5432/todo_list").await.unwrap();

    pool
}

// Con este struct nos comunicaremos con la base de datos
// FromRow es de SQLx, Serialize es para serializar a JSON
#[derive(FromRow, Serialize)]
struct TodoSql {
    id: i32,
    content: String
}

// Para leer las peticiones del cliente
#[derive(Deserialize)]
struct TodoReq {
    content: String
}
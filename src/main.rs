#[allow(unused)]
use actix_web::{web::{self, Path}, App, HttpResponse, HttpServer, Responder};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, Database, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait};
use serde::Deserialize;

mod todo;

use todo::{ActiveModel as TodoModel, Entity as Todo};

#[actix_web::main]
async fn main() {
    let conn = db().await;
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(conn.clone()))
        .route("/todo_list", web::get().to(get_todo_list))
        .route("/add_todo", web::post().to(add_todo))
        .route("/todo/{id}", web::get().to(get_single_todo))
        .route("/delete_todo/{id}", web::delete().to(delete_todo))
        .route("/update_todo/{id}", web::put().to(update_todo))
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    .run()
    .await
    .unwrap()
}

async fn get_todo_list(conn: web::Data<DatabaseConnection>) -> impl Responder {
    let todos = Todo::find().all(conn.get_ref()).await.unwrap();

    let todos_json = serde_json::to_string_pretty(&todos).unwrap();

    HttpResponse::Ok().body(todos_json)
}

async fn add_todo(conn: web::Data<DatabaseConnection>, todo_req: web::Json<TodoRequest>) -> impl Responder {
    let todo = TodoModel {
        content: Set(todo_req.content.clone()),
        ..Default::default()
    };

    todo.insert(conn.get_ref()).await.unwrap();
    HttpResponse::Ok().body("Added new Todo successfully")
}

async fn get_single_todo(conn: web::Data<DatabaseConnection>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let todo_op = Todo::find_by_id(id).one(conn.get_ref()).await.unwrap();

    match todo_op {
        None => {
            let msg = format!("NO todo: {id} Found!");
            HttpResponse::NotFound().body(msg)
        },
        Some(todo) => {
            let todo_json = serde_json::to_string_pretty(&todo).unwrap();
            HttpResponse::Ok().body(todo_json)
        }
    }

}

async fn delete_todo(conn: web::Data<DatabaseConnection>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let todo = Todo::find_by_id(id).one(conn.get_ref()).await.unwrap();

    match todo {
        None => {
            let msg = format!("todo: {id} not found");
            HttpResponse::NotFound().body(msg)
        }
        Some(todo) => {
            todo.delete(conn.get_ref()).await.unwrap();
            HttpResponse::Ok().body("The todo was deleted successfully")
        }
    }
}

async fn update_todo(conn: web::Data<DatabaseConnection>, path: web::Path<i32>, todo_req: web::Json<TodoRequest>) -> impl Responder {
    let id = path.into_inner();
    let todo = Todo::find_by_id(id).one(conn.get_ref()).await.unwrap();

    match todo {
        None => {
            let msg = format!("todo: {id} not found");
            HttpResponse::NotFound().body(msg)
        }
        Some(todo) => {
            let mut todo = todo.into_active_model();
            todo.content = Set(todo_req.content.clone());
            todo.update(conn.get_ref()).await.unwrap();
            HttpResponse::Ok().body("The todo was updated successfully")
        }
    }
}

async fn db() -> DatabaseConnection {
    let conn = Database::connect("postgres://postgres:postgres@localhost:5432/todo_list").await.unwrap();
    conn
}

#[derive(Deserialize)]
struct TodoRequest {
    content: String,
}
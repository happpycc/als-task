use chrono::prelude::Local;
use snowflake::SnowflakeIdBucket;
use serde_json::{from_value, to_writer_pretty, to_value, from_reader};
use std::fs::File;
use serde::{Deserialize, Serialize};
use actix_web::{get, web, web::Data, App, HttpResponse, HttpServer, Responder};
use sqlx::postgres::PgPoolOptions;
use sqlx::{FromRow, Postgres, Pool};

#[derive(Debug, Default, Deserialize, Serialize)]
enum TaskState {
    Abandon,
    Done,
    #[default]
    Todo,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    task_id: i64,
    depth: u8,
    content: String,
    state: TaskState,
    comments: Option<String>,
    create_time: String,
    update_time: String,
    dead_time: Option<String>,
    prev_task: Option<i64>,
    next_task: Option<i64>
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
struct TaskDB {
    task: serde_json::Value
}

struct AppState {
    db: Pool<Postgres>
}

impl Default for TaskDB {
    fn default() -> Self {
        Self {
            task: to_value(Task {..Default::default()}).unwrap()
        }
    }    
}

impl Default for Task {
    fn default() -> Self {
        let mut id_generator_bucket = SnowflakeIdBucket::new(1, 1);
        let local_time = Local::now();
        Self {
            depth: 0,
            task_id: id_generator_bucket.get_id(),
            content: "".to_string(),
            state: TaskState::default(),
            comments: None,
            create_time: local_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            update_time: local_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            dead_time: None,
            prev_task: None,
            next_task: None
        }
    }
}

impl Task {

}

fn import_tasks(path: &str) -> Vec<Task> {
    let f = File::open(path).unwrap();
    from_reader(f).unwrap()
}

fn save_tasks(path: &str, tasks: Vec<Task>) {
    let j = to_value(tasks).unwrap();
    let f = File::create(path).unwrap();
    to_writer_pretty(f, &j).unwrap();
}

#[get("/")]
async fn index(state :Data<AppState>) -> impl Responder {
    match sqlx::query
        ("INSERT INTO tasks (task) VALUES ($1)")
        .bind(TaskDB {..Default::default()}.task)
        .execute(&state.db)
        .await
    {
        Ok(tasks) => println!("{:?}", tasks),
        Err(e) => println!("{:?}", e)
    };
    match sqlx::query_as::<_,  TaskDB>("SELECT * FROM tasks")
        .fetch_all(&state.db)
        .await
    {
        Ok(tasks) => {
            println!("{:?}", tasks);
            HttpResponse::Ok().json(to_value(tasks).unwrap())
        },
        Err(_) => HttpResponse::NotFound().json("No tasks found")
    }
}

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:als@localhost/als_task").await?;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
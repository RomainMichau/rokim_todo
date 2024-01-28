use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use chrono::{DateTime, Utc};
use tokio_postgres::{NoTls, Error, Client};
use serde::Serialize;
use utoipa::ToSchema;


#[derive(Serialize, ToSchema)]
pub struct Todo {
    pub(crate) id: i64,
    pub(crate) description: Option<String>,
    pub(crate) done_at: Option<DateTime<Utc>>,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) category: String,
    pub(crate) title: String,
}


impl Responder for Todo {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

pub(crate) struct DbClient {
    client: Client,
}

impl DbClient {
    pub async fn new() -> Result<DbClient, Error> {
        let (client, connection) =
            tokio_postgres::connect("host=romainmic.com user=rokim_todo_rw password=yKNbvqWjLeuSUySJe5kB dbname=rokim_todo", NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        Ok(DbClient { client })
    }

    pub async fn get_todos(&self) -> Result<Vec<Todo>, Error> {
        let rows = self.client.query("SELECT id, description, done_at, created_at, category, title FROM todos", &[]).await?;
        let mut todos = Vec::new();
        for row in rows {
            let todo = Todo {
                id: row.get(0),
                description: row.get(1),
                done_at: match row.get(2) {
                    Some(done_at) => Some(done_at),
                    None => None,
                },
                created_at: row.get(3),
                category: row.get(4),
                title: row.get(5),
            };
            todos.push(todo);
        }
        Ok(todos)
    }

    pub async fn create_todo(
        &self,
        description: &Option<String>,
        category: &String,
        title: &String,
    ) -> Result<Todo, Error> {
        let row = self.client.query_one(
            "INSERT INTO todos (description, category, title, created_at) VALUES ($1, $2, $3, $4) RETURNING id, description, done_at, created_at, category, title",
            &[&description, &category, &title, &Utc::now()],
        ).await?;
        let todo = Todo {
            id: row.get(0),
            description: row.get(1),
            done_at: match row.get(2) {
                Some(done_at) => Some(done_at),
                None => None,
            },
            created_at: row.get(3),
            category: row.get(4),
            title: row.get(5),
        };
        Ok(todo)
    }

    pub async fn update_todo(
        &self,
        id: i64,
        description: &Option<String>,
        category: &String,
        title: &String,
    ) -> Result<Todo, Error> {
        let row = self.client.query_one(
            "UPDATE todos SET description = $1, category = $2, title = $3 WHERE id = $4 RETURNING id, description, done_at, created_at, category, title",
            &[&description, &category, &title, &id],
        ).await?;
        let todo = Todo {
            id: row.get(0),
            description: row.get(1),
            done_at: match row.get(2) {
                Some(done_at) => Some(done_at),
                None => None,
            },
            created_at: row.get(3),
            category: row.get(4),
            title: row.get(5),
        };
        Ok(todo)
    }

    pub async fn delete_todo(
        &self,
        id: i64,
    ) -> Result<(), Error> {
        self.client.execute(
            "DELETE FROM todos WHERE id = $1",
            &[&id],
        ).await?;
        Ok(())
    }

    pub async fn mark_todo_as_done(
        &self,
        id: i64,
    ) -> Result<Todo, Error> {
        let row = self.client.query_one(
            "UPDATE todos SET done_at = $1 WHERE id = $2 RETURNING id, description, done_at, created_at, category, title",
            &[&Utc::now(), &id],
        ).await?;
        let todo = Todo {
            id: row.get(0),
            description: row.get(1),
            done_at: match row.get(2) {
                Some(done_at) => Some(done_at),
                None => None,
            },
            created_at: row.get(3),
            category: row.get(4),
            title: row.get(5),
        };
        Ok(todo)
    }

    pub async fn mark_todo_as_undone(
        &self,
        id: i64,
    ) -> Result<Todo, Error> {
        let row = self.client.query_one(
            "UPDATE todos SET done_at = $1 WHERE id = $2 RETURNING id, description, done_at, created_at, category, title",
            &[&None::<DateTime<Utc>>, &id],
        ).await?;
        let todo = Todo {
            id: row.get(0),
            description: row.get(1),
            done_at: match row.get(2) {
                Some(done_at) => Some(done_at),
                None => None,
            },
            created_at: row.get(3),
            category: row.get(4),
            title: row.get(5),
        };
        Ok(todo)
    }

}
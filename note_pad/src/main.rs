use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use std::sync::Arc;
use tokio::net::TcpListener;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Note {
    id: Uuid,
    title: String,
    content: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct CreateNote {
    title: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct UpdateNote {
    title: Option<String>,
    content: Option<String>,
}

struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    let database_url = "postgres://postgres:rebecca@localhost/note_pad";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to database");

    let app_state = Arc::new(AppState { db: pool });

    let app = Router::new()
        .route("/api/v1/healthcheck", get(health_check_handler))
        .route("/api/v1/notes", get(get_notes).post(create_note))
        .route("/api/v1/notes/{id}", get(get_note).put(update_note).delete(delete_note))
        .with_state(app_state);

    println!("Server started successfully at 0.0.0.0:8080");

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Note Pad API Services";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}

async fn get_notes(
    State(state): State<Arc<AppState>>,
    Query(params): Query<std::collections::HashMap<String, i32>>,
) -> Result<Json<Vec<Note>>, (StatusCode, String)> {
    let limit = params.get("limit").unwrap_or(&10).clone();
    let offset = params.get("offset").unwrap_or(&0).clone();

    let rows = sqlx::query("SELECT * FROM notes ORDER BY created_at DESC LIMIT $1 OFFSET $2")
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut notes = Vec::new();
    for row in rows {
        let note = Note {
            id: row.try_get("id").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
            title: row.try_get("title").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
            content: row.try_get("content").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
            created_at: row.try_get("created_at").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
            updated_at: row.try_get("updated_at").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        };
        notes.push(note);
    }

    Ok(Json(notes))
}

async fn get_note(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Note>, (StatusCode, String)> {
    let row = sqlx::query("SELECT * FROM notes WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Note not found".to_string()))?;

    let note = Note {
        id: row.try_get("id").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        title: row.try_get("title").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        content: row.try_get("content").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        created_at: row.try_get("created_at").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        updated_at: row.try_get("updated_at").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
    };

    Ok(Json(note))
}

async fn create_note(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateNote>,
) -> Result<Json<Note>, (StatusCode, String)> {
    let row = sqlx::query(
        "INSERT INTO notes (title, content) VALUES ($1, $2) RETURNING id, title, content, created_at, updated_at"
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let note = Note {
        id: row.try_get("id").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        title: row.try_get("title").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        content: row.try_get("content").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        created_at: row.try_get("created_at").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        updated_at: row.try_get("updated_at").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
    };

    Ok(Json(note))
}

async fn update_note(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateNote>,
) -> Result<Json<Note>, (StatusCode, String)> {
    let row = sqlx::query(
        "UPDATE notes SET title = COALESCE($1, title), content = COALESCE($2, content), updated_at = NOW() WHERE id = $3 RETURNING id, title, content, created_at, updated_at"
    )
    .bind(payload.title)
    .bind(payload.content)
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "Note not found".to_string()))?;

    let note = Note {
        id: row.try_get("id").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        title: row.try_get("title").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        content: row.try_get("content").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        created_at: row.try_get("created_at").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        updated_at: row.try_get("updated_at").map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
    };

    Ok(Json(note))
}

async fn delete_note(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM notes WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Note not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
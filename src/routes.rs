//! REST handlers for `template-dtnpm-mod`.

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use dtnpm_web_core::ModuleContext;
use serde::{Deserialize, Serialize};
use serde_json::json;
use time::OffsetDateTime;
use tracing::info;

/// Structure representing a persisted module item.
#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleItem {
    pub id: String,
    pub source: String,
    pub destination: String,
    pub title: String,
    pub content: String,
    pub created_at: i64,
    pub is_valid_bpsec: bool,
}

/// Request body for creating a new module item.
#[derive(Debug, Deserialize)]
pub struct CreateItemRequest {
    pub destination: String,
    pub title: String,
    pub content: String,
}

fn internal_error<E: std::fmt::Display>(err: E) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": err.to_string() })),
    )
}

fn bad_request(msg: &str) -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::BAD_REQUEST, Json(json!({ "error": msg })))
}

/// GET /items - List all items.
pub async fn list_items(
    State(ctx): State<ModuleContext>,
) -> Result<Json<Vec<ModuleItem>>, (StatusCode, Json<serde_json::Value>)> {
    let conn_lock = ctx.internal_storage.connection();
    let conn = conn_lock
        .lock()
        .map_err(|_| internal_error("Failed to acquire database lock"))?;

    let mut stmt = conn
        .prepare(
            "SELECT id, source, destination, title, content, created_at, is_valid_bpsec
             FROM module_items ORDER BY created_at DESC LIMIT 100",
        )
        .map_err(internal_error)?;

    let rows = stmt
        .query_map([], |row| {
            let is_valid: i64 = row.get(6)?;
            Ok(ModuleItem {
                id: row.get(0)?,
                source: row.get(1)?,
                destination: row.get(2)?,
                title: row.get(3)?,
                content: row.get(4)?,
                created_at: row.get(5)?,
                is_valid_bpsec: is_valid != 0,
            })
        })
        .map_err(internal_error)?;

    let mut items = Vec::new();
    for item in rows {
        items.push(item.map_err(internal_error)?);
    }

    Ok(Json(items))
}

/// POST /items - Create a new item.
pub async fn create_item(
    State(ctx): State<ModuleContext>,
    Json(payload): Json<CreateItemRequest>,
) -> Result<(StatusCode, Json<ModuleItem>), (StatusCode, Json<serde_json::Value>)> {
    if payload.destination.trim().is_empty() {
        return Err(bad_request("destination cannot be empty"));
    }
    if payload.title.trim().is_empty() {
        return Err(bad_request("title cannot be empty"));
    }
    if payload.content.len() > 1_000_000 {
        return Err(bad_request("content exceeds 1MB limit"));
    }

    let item_id = format!("item-{}", OffsetDateTime::now_utc().unix_timestamp_nanos());
    let source = ctx.config.node_id.clone();
    let now = OffsetDateTime::now_utc().unix_timestamp();

    {
        let conn_lock = ctx.internal_storage.connection();
        let conn = conn_lock
            .lock()
            .map_err(|_| internal_error("Failed to acquire database lock"))?;

        conn.execute(
            "INSERT INTO module_items (id, source, destination, title, content, created_at, is_valid_bpsec)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                &item_id,
                &source,
                &payload.destination,
                &payload.title,
                &payload.content,
                now,
                1,
            ),
        )
        .map_err(internal_error)?;
    }

    info!(item_id = %item_id, destination = %payload.destination, "Created new module item");

    let item = ModuleItem {
        id: item_id,
        source,
        destination: payload.destination,
        title: payload.title,
        content: payload.content,
        created_at: now,
        is_valid_bpsec: true,
    };

    Ok((StatusCode::CREATED, Json(item)))
}

/// GET /items/{id} - Get a single item by id.
pub async fn get_item(
    State(ctx): State<ModuleContext>,
    Path(id): Path<String>,
) -> Result<Json<ModuleItem>, (StatusCode, Json<serde_json::Value>)> {
    let conn_lock = ctx.internal_storage.connection();
    let conn = conn_lock
        .lock()
        .map_err(|_| internal_error("Failed to acquire database lock"))?;

    let mut stmt = conn
        .prepare(
            "SELECT id, source, destination, title, content, created_at, is_valid_bpsec
             FROM module_items WHERE id = ?1",
        )
        .map_err(internal_error)?;

    let item = stmt
        .query_row([&id], |row| {
            let is_valid: i64 = row.get(6)?;
            Ok(ModuleItem {
                id: row.get(0)?,
                source: row.get(1)?,
                destination: row.get(2)?,
                title: row.get(3)?,
                content: row.get(4)?,
                created_at: row.get(5)?,
                is_valid_bpsec: is_valid != 0,
            })
        })
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Item not found" })),
            ),
            other => internal_error(other),
        })?;

    Ok(Json(item))
}

/// DELETE /items/{id} - Delete an item by id.
pub async fn delete_item(
    State(ctx): State<ModuleContext>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let conn_lock = ctx.internal_storage.connection();
    let conn = conn_lock
        .lock()
        .map_err(|_| internal_error("Failed to acquire database lock"))?;

    let count = conn
        .execute("DELETE FROM module_items WHERE id = ?1", [&id])
        .map_err(internal_error)?;

    if count == 0 {
        Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Item not found" })),
        ))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

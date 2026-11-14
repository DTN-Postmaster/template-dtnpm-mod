use std::sync::Arc;

use axum::http::StatusCode;
use dtnpm_web_core::TestServerBuilder;
use serde_json::Value;
use template_dtnpm_mod::TemplateModule;
use template_dtnpm_mod::routes::ModuleItem;

#[tokio::test]
async fn test_module_integration_lifecycle() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug,hardy_bpa=trace,template_dtnpm_mod=trace")
        .try_init();

    let srv = TestServerBuilder::new()
        .node_id("dtn://node-a/")
        .callsign("NODEA")
        .with_module(Arc::new(TemplateModule::new()))
        .build()
        .await
        .expect("TestServerBuilder should succeed");

    // 1. Verify capabilities endpoint lists our module
    let caps_res = srv.get("/api/capabilities").await;
    assert_eq!(caps_res.status(), StatusCode::OK);
    let caps_bytes = axum::body::to_bytes(caps_res.into_body(), 1024 * 1024)
        .await
        .expect("body");
    let caps: Value = serde_json::from_slice(&caps_bytes).expect("json");
    assert_eq!(caps["modules"][0]["id"], "template_mod");

    // 2. Initial item list is empty
    let res = srv.get("/api/template_mod/v1/items").await;
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(res.into_body(), 1024 * 1024)
        .await
        .expect("body");
    let items: Vec<ModuleItem> = serde_json::from_slice(&bytes).expect("json");
    assert!(items.is_empty());

    // 3. Create an item
    let create_payload = serde_json::json!({
        "destination": "dtn://node-b/template",
        "title": "Test Title",
        "content": "Hello from automated test!"
    });
    let res = srv
        .post_json("/api/template_mod/v1/items", create_payload)
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);
    let bytes = axum::body::to_bytes(res.into_body(), 1024 * 1024)
        .await
        .expect("body");
    let item: ModuleItem = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(item.title, "Test Title");
    assert_eq!(item.destination, "dtn://node-b/template");
    let item_id = item.id.clone();

    // 4. Fetch the item by id
    let res = srv
        .get(&format!("/api/template_mod/v1/items/{}", item_id))
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    // 5. Delete the item
    let req = axum::http::Request::builder()
        .method("DELETE")
        .uri(format!("/api/template_mod/v1/items/{}", item_id))
        .body(axum::body::Body::empty())
        .expect("delete request");
    let res = srv.oneshot(req).await;
    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    // 6. Fetching deleted item returns 404
    let res = srv
        .get(&format!("/api/template_mod/v1/items/{}", item_id))
        .await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_module_validation_rejections() {
    let srv = TestServerBuilder::new()
        .node_id("dtn://node-a/")
        .with_module(Arc::new(TemplateModule::new()))
        .build()
        .await
        .expect("TestServerBuilder should succeed");

    // Empty destination rejected
    let res = srv
        .post_json(
            "/api/template_mod/v1/items",
            serde_json::json!({
                "destination": "",
                "title": "Title",
                "content": "Content"
            }),
        )
        .await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    // Empty title rejected
    let res = srv
        .post_json(
            "/api/template_mod/v1/items",
            serde_json::json!({
                "destination": "dtn://dest/",
                "title": "",
                "content": "Content"
            }),
        )
        .await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

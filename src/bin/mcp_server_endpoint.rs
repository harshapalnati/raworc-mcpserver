use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::post,
    Json, Router,
};
use dotenvy::dotenv;
use serde_json::{json, Value};
use std::{env, net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState {
    api_token: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_token = env::var("API_TOKEN").unwrap_or_else(|_| "dev-token-change-me".to_string());

    let state = Arc::new(AppState { api_token });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let app = Router::new()
        .route("/mcp", post(mcp_handler))
        .with_state(state)
        .layer(cors);

    let port: u16 = env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8787);
    let addr: SocketAddr = ([0, 0, 0, 0], port).into();
    println!("MCP server listening on http://{addr}/mcp");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn mcp_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Auth: Bearer <token>
    let auth_ok = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|tok| tok == state.api_token)
        .unwrap_or(false);

    if !auth_ok {
        return Err((StatusCode::UNAUTHORIZED, "invalid or missing token".into()));
    }

    // Basic JSON-RPC 2.0 routing
    let jsonrpc = payload.get("jsonrpc").and_then(|v| v.as_str()).unwrap_or("");
    let id = payload.get("id").cloned().unwrap_or(json!(null));
    let method = payload.get("method").and_then(|v| v.as_str()).unwrap_or("");
    let params = payload.get("params").cloned().unwrap_or(json!({}));

    if jsonrpc != "2.0" {
        return Ok(Json(json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": { "code": -32600, "message": "Invalid Request: jsonrpc must be '2.0'" }
        })));
    }

    let result = match method {
        "initialize" => {
            json!({
                "serverInfo": { "name": "rust-mcp-server", "version": "0.1.0" },
                "protocolVersion": "2025-06-18"
            })
        }
        "tools/list" => {
            json!({
                "tools": [
                    {
                        "name": "echo",
                        "description": "Echo back provided text",
                        "inputSchema": {
                            "type": "object",
                            "properties": { "text": { "type": "string" } },
                            "required": ["text"]
                        }
                    }
                ]
            })
        }
        "tools/call" => {
            let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let args = params.get("arguments").cloned().unwrap_or(json!({}));
            if name == "echo" {
                let text = args.get("text").and_then(|v| v.as_str()).unwrap_or("");
                json!({ "content": [{ "type": "text", "text": text }] })
            } else {
                json!({ "error": { "code": -32601, "message": format!("Unknown tool: {name}") } })
            }
        }
        _ => json!({ "error": { "code": -32601, "message": format!("Method not found: {method}") } }),
    };

    Ok(Json(json!({ "jsonrpc": "2.0", "id": id, "result": result })))
}



//! Verifies runtime tick transitions for successful, failed, and disconnected requests.

use serde_json::json;
use stellar_tui::{
    app::App,
    rpc::types::{RpcError, RpcResponse},
    settings::Settings,
};

#[test]
fn tick_handles_success_response_and_sets_completed_status() {
    let mut app = App::new(Settings::default_settings());
    let (tx, rx) = std::sync::mpsc::channel();
    app.pending_request = Some(rx);
    app.status = "Calling...".to_string();

    tx.send(Ok(RpcResponse {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: Some(json!({ "ok": true })),
        error: None,
    }))
    .expect("send should succeed");

    app.tick();

    assert!(app.pending_request.is_none());
    assert_eq!(app.status, "Ready");
    assert_eq!(
        app.timed_status
            .as_ref()
            .expect("timed status should be set")
            .message,
        "Completed"
    );
    assert!(app.last_response.contains("\"ok\": true"));
    assert!(app.paginated_response.is_some());
}

#[test]
fn tick_handles_rpc_error_payload() {
    let mut app = App::new(Settings::default_settings());
    let (tx, rx) = std::sync::mpsc::channel();
    app.pending_request = Some(rx);

    tx.send(Ok(RpcResponse {
        jsonrpc: "2.0".to_string(),
        id: 1,
        result: None,
        error: Some(RpcError {
            code: -32602,
            message: "Invalid params".to_string(),
        }),
    }))
    .expect("send should succeed");

    app.tick();

    assert_eq!(app.status, "Ready");
    assert!(app
        .timed_status
        .as_ref()
        .expect("timed status should be set")
        .message
        .contains("Error -32602: Invalid params"));
    assert!(app.last_response.contains("Invalid params"));
}

#[test]
fn tick_handles_request_transport_error_and_clears_response_state() {
    let mut app = App::new(Settings::default_settings());
    let (tx, rx) = std::sync::mpsc::channel();
    app.pending_request = Some(rx);
    app.last_response = "previous response".to_string();
    app.response_search_query = "prev".to_string();
    app.response_search_matches = vec![0];
    app.response_page = 2;
    app.response_scroll = 5;

    tx.send(Err("network down".to_string()))
        .expect("send should succeed");

    app.tick();

    assert!(app.pending_request.is_none());
    assert_eq!(app.status, "Ready");
    assert!(app.last_response.is_empty());
    assert!(app.response_search_query.is_empty());
    assert!(app.response_search_matches.is_empty());
    assert_eq!(app.response_page, 0);
    assert_eq!(app.response_scroll, 0);
    assert!(app
        .timed_status
        .as_ref()
        .expect("timed status should be set")
        .message
        .contains("Request failed: network down"));
}

#[test]
fn tick_handles_disconnected_channel() {
    let mut app = App::new(Settings::default_settings());
    let (tx, rx) = std::sync::mpsc::channel::<Result<RpcResponse<serde_json::Value>, String>>();
    app.pending_request = Some(rx);
    drop(tx);

    app.tick();

    assert!(app.pending_request.is_none());
    assert_eq!(app.status, "Ready");
    assert_eq!(
        app.timed_status
            .as_ref()
            .expect("timed status should be set")
            .message,
        "Request failed: channel closed"
    );
}

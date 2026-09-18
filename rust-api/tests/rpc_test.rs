use cubase_ecs_api::{cubase::dispatcher::Dispatcher, rpc::handler::handle_rpc};
use serde_json::json;

#[tokio::test]
async fn mixer_set_ok() {
    let d = Dispatcher::mock();
    let req = json!({"jsonrpc":"2.0","method":"mixer.set","params":{"track":"Vocal","param":"volume","value":-6.0},"id":1});
    let resp = handle_rpc(req, &d).await.unwrap();
    assert_eq!(resp["result"]["ok"], true);
    assert_eq!(resp["result"]["method"], "mixer.set");
    assert_eq!(resp["id"], 1);
}

#[tokio::test]
async fn plugin_set_param_ok() {
    let d = Dispatcher::mock();
    let req = json!({"jsonrpc":"2.0","method":"plugin.set_param","params":{"track":"Vocal","slot":2,"plugin":"Compressor","param":"Threshold","value":-18.0},"id":2});
    let resp = handle_rpc(req, &d).await.unwrap();
    assert_eq!(resp["result"]["ok"], true);
    assert_eq!(resp["id"], 2);
}

#[tokio::test]
async fn command_exec_ok() {
    let d = Dispatcher::mock();
    let req =
        json!({"jsonrpc":"2.0","method":"command.exec","params":{"id":"Transport_Play"},"id":3});
    let resp = handle_rpc(req, &d).await.unwrap();
    assert_eq!(resp["result"]["ok"], true);
}

#[tokio::test]
async fn session_status_ok() {
    let d = Dispatcher::mock();
    let req = json!({"jsonrpc":"2.0","method":"session.status","params":{},"id":4});
    let resp = handle_rpc(req, &d).await.unwrap();
    assert_eq!(resp["result"]["ok"], true);
}

#[tokio::test]
async fn unknown_method_returns_method_not_found() {
    let d = Dispatcher::mock();
    let req = json!({"jsonrpc":"2.0","method":"nope","params":{},"id":9});
    let resp = handle_rpc(req, &d).await.unwrap();
    assert_eq!(resp["error"]["code"], -32601);
}

#[tokio::test]
async fn invalid_params_returns_invalid_params() {
    let d = Dispatcher::mock();
    // value欠落
    let req = json!({"jsonrpc":"2.0","method":"mixer.set","params":{"track":"Vocal","param":"volume"},"id":10});
    let err = handle_rpc(req, &d).await.unwrap_err();
    assert_eq!(err.1["error"]["code"], -32602);
}

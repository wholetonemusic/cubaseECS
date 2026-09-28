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

#[tokio::test]
async fn mixer_set_rejects_unknown_param() {
    let d = Dispatcher::mock();
    let req = json!({"jsonrpc":"2.0","method":"mixer.set","params":{"track":"Vocal","param":"reverb","value":0.5},"id":11});
    let err = handle_rpc(req, &d).await.unwrap_err();
    assert_eq!(err.1["error"]["code"], -32602);
}

#[tokio::test]
async fn mixer_set_rejects_out_of_range_values() {
    let d = Dispatcher::mock();
    for (param, value) in [("volume01", 1.5), ("volume01", -0.1), ("pan", 2.0)] {
        let req = json!({"jsonrpc":"2.0","method":"mixer.set","params":{"track":"Vocal","param":param,"value":value},"id":12});
        let err = handle_rpc(req, &d).await.unwrap_err();
        assert_eq!(
            err.1["error"]["code"], -32602,
            "param={param} value={value}"
        );
    }
}

#[tokio::test]
async fn mixer_set_rejects_empty_track() {
    let d = Dispatcher::mock();
    let req = json!({"jsonrpc":"2.0","method":"mixer.set","params":{"track":"  ","param":"volume","value":-6.0},"id":13});
    let err = handle_rpc(req, &d).await.unwrap_err();
    assert_eq!(err.1["error"]["code"], -32602);
}

#[tokio::test]
async fn command_exec_rejects_unknown_id() {
    let d = Dispatcher::mock();
    let req =
        json!({"jsonrpc":"2.0","method":"command.exec","params":{"id":"Transport_Fly"},"id":14});
    let err = handle_rpc(req, &d).await.unwrap_err();
    assert_eq!(err.1["error"]["code"], -32602);
}

#[tokio::test]
async fn command_exec_accepts_known_aliases() {
    let d = Dispatcher::mock();
    for cmd in [
        "Transport_Play",
        "Transport_Stop",
        "Transport_Record",
        "play",
        "stop",
    ] {
        let req = json!({"jsonrpc":"2.0","method":"command.exec","params":{"id":cmd},"id":15});
        let resp = handle_rpc(req, &d).await.unwrap();
        assert_eq!(resp["result"]["ok"], true, "cmd={cmd}");
    }
}

#[tokio::test]
async fn plugin_set_param_rejects_empty_param_name() {
    let d = Dispatcher::mock();
    let req = json!({"jsonrpc":"2.0","method":"plugin.set_param","params":{"track":"Vocal","slot":0,"plugin":"Compressor","param":"","value":0.5},"id":16});
    let err = handle_rpc(req, &d).await.unwrap_err();
    assert_eq!(err.1["error"]["code"], -32602);
}

#[tokio::test]
async fn session_status_reports_local_info() {
    let d = Dispatcher::mock();
    let req = json!({"jsonrpc":"2.0","method":"session.status","params":{},"id":17});
    let resp = handle_rpc(req, &d).await.unwrap();
    assert_eq!(resp["result"]["ok"], true);
    assert_eq!(resp["result"]["service"], "cubase-ecs-api");
    assert_eq!(resp["result"]["midi_mode"], "mock");
}

#[tokio::test]
async fn send_set_ok() {
    let d = Dispatcher::mock();
    let req = json!({"jsonrpc":"2.0","method":"send.set","params":{"track":"Vocal","slot":0,"param":"level","value":0.5},"id":18});
    let resp = handle_rpc(req, &d).await.unwrap();
    assert_eq!(resp["result"]["ok"], true);
    assert_eq!(resp["result"]["method"], "send.set");
    assert_eq!(resp["id"], 18);
}

#[tokio::test]
async fn send_set_rejects_bad_param_and_range() {
    let d = Dispatcher::mock();
    let bad_param = json!({"jsonrpc":"2.0","method":"send.set","params":{"track":"Vocal","slot":0,"param":"destination","value":0.5},"id":19});
    assert_eq!(
        handle_rpc(bad_param, &d).await.unwrap_err().1["error"]["code"],
        -32602
    );
    let bad_range = json!({"jsonrpc":"2.0","method":"send.set","params":{"track":"Vocal","slot":0,"param":"level","value":1.5},"id":20});
    assert_eq!(
        handle_rpc(bad_range, &d).await.unwrap_err().1["error"]["code"],
        -32602
    );
}

#[tokio::test]
async fn analyzer_get_features_mock_ok() {
    // テスト時は MIDI_MODE 未設定 → mock合成値。
    let d = Dispatcher::mock();
    let req = json!({"jsonrpc":"2.0","method":"analyzer.get_features","params":{},"id":21});
    let resp = handle_rpc(req, &d).await.unwrap();
    let f = &resp["result"];
    assert_eq!(f["mode"], "mock");
    assert_eq!(f["stale"], false);
    assert!(f["rms"].is_number());
    assert!(f["bands"]["low"].is_number());
    assert!(f["bands"]["high"].is_number());
    assert!(f["transient"].is_number());
    assert!(f["stereo_width"].is_number());
    assert!(f["timestamp"].is_number());
    assert_eq!(resp["id"], 21);
}

#[tokio::test]
async fn analyzer_get_features_rejects_non_object_params() {
    let d = Dispatcher::mock();
    let req = json!({"jsonrpc":"2.0","method":"analyzer.get_features","params":[1,2],"id":22});
    let err = handle_rpc(req, &d).await.unwrap_err();
    assert_eq!(err.1["error"]["code"], -32602);
}

#[test]
fn analyzer_layout_matches_vst_side() {
    // rust-analyzer/src/features.rs との同期保証 (手動同期。値の変更は両側へ)。
    assert_eq!(cubase_ecs_api::analyzer::SHM_SIZE, 1024);
    assert_eq!(
        cubase_ecs_api::analyzer::SHM_OS_ID,
        "CubaseECS_AudioFeatures_v1"
    );
    assert_eq!(cubase_ecs_api::analyzer::FEATURES_MAGIC, 0x4543_5346);
    assert_eq!(cubase_ecs_api::analyzer::FEATURES_VERSION, 1);
}

#[cfg(feature = "wasm")]
ow_wasm_action::pass_json!(func);

#[cfg(not(feature = "wasm"))]
ow_wasm_action::json_args!(func);

fn func(json: serde_json::Value) -> Result<serde_json::Value, anyhow::Error> {
    bench_disk_seq::main(json).map_err(|e| anyhow::anyhow!("{e}"))
}

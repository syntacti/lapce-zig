
use lapce_plugin::{register_plugin, start_lsp, LapcePlugin};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

#[derive(Default)]
struct State {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    arch: String,
    os: String,
    configuration: Configuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    language_id: String,
    options: Option<Value>,
}

register_plugin!(State);

impl LapcePlugin for State {
    fn initialize(&mut self, info: serde_json::Value) {
        eprintln!("Starting lapce-zig plugin!");
        let info = serde_json::from_value::<PluginInfo>(info).unwrap();
        let arch = match info.arch.as_str() {
            "x86_64" => "x86_64",
            "aarch64" => "aarch64",
            _ => return,
        };
        let os = match info.os.as_str() {
            "linux" => "unknown-linux-gnu",
            "macos" => "apple-darwin",
            "windows" => "pc-windows-msvc",
            _ => return,
        };

        // lapce-rust does this; is there any handler?
        // let lock_file = PathBuf::from("download.lock");
        // send_notification(
        //     "python",
        //     &json!({
        //         "path": &lock_file,
        //     }),
        // );

        let zls_path = match env::var("ZLS_PATH") {
            Ok(var) => var,
            Err(error) => panic!("Couldn't get PYLS_PATH: {error}"),
        };

        serde_json::to_writer_pretty(std::io::stderr(), &info).unwrap();
            start_lsp(&zls_path, "zig", info.configuration.options);
    }
}

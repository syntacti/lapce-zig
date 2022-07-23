use lapce_plugin::{register_plugin, start_lsp, LapcePlugin};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    system_lsp: bool,
    enabled: bool,
    options: Option<Value>,
}

register_plugin!(State);

impl LapcePlugin for State {
    fn initialize(&mut self, info: serde_json::Value) {
        eprintln!("Starting lapce-zig plugin!");
        let info = serde_json::from_value::<PluginInfo>(info).unwrap();

        if info.configuration.enabled {
            let exec_path = if info.configuration.system_lsp {
                "zls".to_string()
            } else {
                "".to_string()
            };

        serde_json::to_writer_pretty(std::io::stderr(), &info).unwrap();
        start_lsp(
           &exec_path,
            info.configuration.language_id.as_str(),
            info.configuration.options,
            info.configuration.system_lsp,
        );
    }
}
}

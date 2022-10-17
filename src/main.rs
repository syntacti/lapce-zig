use std::{path::Path, fs};

use anyhow::Result;
use lapce_plugin::{
    psp_types::{
        lsp_types::{request::Initialize, DocumentFilter, DocumentSelector, InitializeParams, Url},
        Request,
    },
    register_plugin, LapcePlugin, PLUGIN_RPC, Http,
};
use serde_json::Value;

#[derive(Default)]
struct State {}

register_plugin!(State);

fn initialize(params: InitializeParams) -> Result<()> {
    PLUGIN_RPC.stderr("lapce-zig");
    download_zls()?;
    let document_selector: DocumentSelector = vec![DocumentFilter {
        language: Some(String::from("zig")),
        pattern: Some(String::from("**.zig")),
        scheme: None,
    }];
    let mut server_args = vec![];

    if let Some(options) = params.initialization_options.as_ref() {
        if let Some(lsp) = options.get("lsp") {
            if let Some(args) = lsp.get("serverArgs") {
                if let Some(args) = args.as_array() {
                    if !args.is_empty() {
                        server_args = vec![];
                    }
                    for arg in args {
                        if let Some(arg) = arg.as_str() {
                            server_args.push(arg.to_string());
                        }
                    }
                }
            }

            if let Some(server_path) = lsp.get("serverPath") {
                if let Some(server_path) = server_path.as_str() {
                    if !server_path.is_empty() {
                        let url = Url::parse(&format!("urn:{}", server_path))?;
                        PLUGIN_RPC.start_lsp(
                            url,
                            server_args,
                            document_selector,
                            params.initialization_options,
                        );
                        return Ok(());
                    }
                }
            }
        }
    }

    // let server_path = Url::parse("urn:zls")?;

    let volt_uri = std::env::var("VOLT_URI")?;
    let server_path = Url::parse(&volt_uri)
        .unwrap()
        .join("zls")
        .unwrap();
    PLUGIN_RPC.start_lsp(
        server_path,
        server_args,
        document_selector,
        params.initialization_options,
    );

    Ok(())
}

impl LapcePlugin for State {
    fn handle_request(&mut self, _id: u64, method: String, params: Value) {
        #[allow(clippy::single_match)]
        match method.as_str() {
            Initialize::METHOD => {
                let params: InitializeParams = serde_json::from_value(params).unwrap();
                if let Err(e) = initialize(params) {
                    PLUGIN_RPC.stderr(&format!("plugin returned with error: {e}"))
                }
            }
            _ => {}
        }
    }
}

fn download_zls() -> Result<bool> {
    const DOWNLOADS_ROOT: &str = "https://zig.pm/zls/downloads";
    let arch = match std::env::var("VOLT_ARCH").as_deref() {
        Ok("x86_64") => "x86_x64",
        Ok("aarch64") => "aarch64",
        _ => panic!("unknow arch"),
    };
    let os = match std::env::var("VOLT_OS").as_deref() {
        Ok("linux") => "linux",
        Ok("macos") => "macos",
        Ok("windows") => "windows",
        _ => panic!("unknow os"),
    };
    
    let lapce_zls_base_name = format!("zls");
    let lapce_zls_path = Path::new(&lapce_zls_base_name);

    if !lapce_zls_path.exists() {
        let volt_download_url = format!(
            "{}/{}-{}/bin/zls",
            &DOWNLOADS_ROOT, &arch,&os
        );
        PLUGIN_RPC.stderr(&format!("Download_URL {}", volt_download_url));

        let mut resp = Http::get(&volt_download_url)?;
        let body = resp.body_read_all()?;
        fs::write(&lapce_zls_path, body)?;
    } else {
        PLUGIN_RPC.stderr("zls already exists");
    }

    Ok(true)
}

// const LANGUAGE_ID: &str = "zig";
// const DOWNLOADS_ROOT: &str = "https://zig.pm/zls/downloads";

// fn initialize(params: InitializeParams) -> Result<()> {
//     eprintln!("starting zig server");
//     let mut server_args = vec![];

//     // Check for user specified LSP server path
//     // ```
//     // [lapce-plugin-name.lsp]
//     // serverPath = "[path or filename]"
//     // serverArgs = ["--arg1", "--arg2"]
//     // ```
//     if let Some(options) = params.initialization_options.as_ref() {
//         if let Some(lsp) = options.get("lsp") {
//             if let Some(args) = lsp.get("serverArgs") {
//                 if let Some(args) = args.as_array() {
//                     for arg in args {
//                         if let Some(arg) = arg.as_str() {
//                             server_args.push(arg.to_string());
//                         }
//                     }
//                 }
//             }

//             if let Some(server_path) = lsp.get("serverPath") {
//                 if let Some(server_path) = server_path.as_str() {
//                     if !server_path.is_empty() {
//                         PLUGIN_RPC.start_lsp(
//                             Url::parse(&format!("urn:{}", server_path))?,
//                             server_args,
//                             LANGUAGE_ID,
//                             params.initialization_options,
//                         );
//                         return Ok(());
//                     }
//                 }
//             }
//         }
//     }

//     // Architecture check
//     let arch = match VoltEnvironment::architecture().as_deref() {
//         Ok("x86_64") => "x86_64",
//         Ok("aarch64") => "aarch64",
//         _ => return Ok(()),
//     };

//     // OS check
//     let os = match VoltEnvironment::operating_system().as_deref() {
//         Ok("macos") => "macos",
//         Ok("linux") => "linux",
//         Ok("windows") => "windows",
//         _ => return Ok(()),
//     };

//     // Download URL
//     let filename = format!("{DOWNLOADS_ROOT}/releases/download/");

//     // see lapce_plugin::Http for available API to download files

//     let _ = match VoltEnvironment::operating_system().as_deref() {
//         Ok("windows") => {
//             format!("{}.exe", "zls")
//         }
//         _ => "zls".to_string(),
//     };

//     // Plugin working directory
//     let volt_uri = VoltEnvironment::uri()?;
//     let server_path = Url::parse(&volt_uri)?.join("zls")?;
//     eprintln!("server path is ready");
//     // Available language IDs
//     // https://github.com/lapce/lapce/blob/HEAD/lapce-proxy/src/buffer.rs#L173
//     PLUGIN_RPC.start_lsp(
//         server_path,
//         server_args,
//         LANGUAGE_ID,
//         params.initialization_options,
//     );

//     Ok(())
// }

// impl LapcePlugin for State {
//     fn handle_request(&mut self, _id: u64, method: String, params: Value) {
//         #[allow(clippy::single_match)]
//         match method.as_str() {
//             Initialize::METHOD => {
//                 let params: InitializeParams = serde_json::from_value(params).unwrap();
//                 let _ = initialize(params);
//             }
//             _ => {}
//         }
//     }
// }

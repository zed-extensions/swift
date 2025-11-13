mod language_server;

use std::collections::HashMap;

use language_server::SourceKitLsp;

use serde::{Deserialize, Serialize};
use zed::settings::LspSettings;
use zed_extension_api::{
    self as zed,
    lsp::{Completion, Symbol},
    serde_json, CodeLabel, LanguageServerId, Result, StartDebuggingRequestArgumentsRequest,
};

#[derive(Default)]
struct SwiftExtension {
    sourcekit_lsp: Option<SourceKitLsp>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SwiftDebugConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cwd: Option<String>,
    #[serde(default)]
    env: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    program: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pid: Option<u32>,
    request: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    stop_on_entry: Option<bool>,
}

impl SwiftExtension {
    const ADAPTER_NAME: &str = "Swift";
}

impl zed::Extension for SwiftExtension {
    fn new() -> Self {
        Self::default()
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            SourceKitLsp::SERVER_ID => {
                let lsp = self.sourcekit_lsp.get_or_insert_with(SourceKitLsp::new);
                lsp.language_server_command(language_server_id, worktree)
            }
            _ => Err(format!("Unknown language server: {}", language_server_id)),
        }
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let initialization_options =
            LspSettings::for_worktree(language_server_id.as_ref(), worktree)
                .ok()
                .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
                .unwrap_or_default();

        Ok(Some(serde_json::json!(initialization_options)))
    }

    fn label_for_completion(
        &self,
        language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
        match language_server_id.as_ref() {
            SourceKitLsp::SERVER_ID => self
                .sourcekit_lsp
                .as_ref()?
                .label_for_completion(completion),
            _ => None,
        }
    }

    fn label_for_symbol(
        &self,
        language_server_id: &LanguageServerId,
        symbol: Symbol,
    ) -> Option<CodeLabel> {
        match language_server_id.as_ref() {
            SourceKitLsp::SERVER_ID => self.sourcekit_lsp.as_ref()?.label_for_symbol(symbol),
            _ => None,
        }
    }

    fn get_dap_binary(
        &mut self,
        adapter_name: String,
        config: zed_extension_api::DebugTaskDefinition,
        user_provided_debug_adapter_path: Option<String>,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<zed_extension_api::DebugAdapterBinary, String> {
        if adapter_name != Self::ADAPTER_NAME {
            return Err(format!("Cannot create binary for adapter: {adapter_name}"));
        }

        let configuration = config.config.to_string();
        let config: SwiftDebugConfig =
            serde_json::from_str(&config.config).map_err(|e| e.to_string())?;
        let request = match config.request.as_str() {
            "launch" => StartDebuggingRequestArgumentsRequest::Launch,
            "attach" => StartDebuggingRequestArgumentsRequest::Attach,
            other => return Err(format!("Unexpected value for `request` key in Swift debug adapter configuration: {other:?}"))
        };
        let (command, arguments) = user_provided_debug_adapter_path
            .map(|path| (path, Vec::<String>::new()))
            .or_else(|| {
                let swiftly = worktree.which("swiftly")?;
                Some((swiftly, vec!["run".into(), "lldb-dap".into()]))
            })
            .or_else(|| {
                let xcrun = worktree.which("xcrun")?;
                Some((xcrun, vec!["lldb-dap".into()]))
            })
            .or_else(|| {
                let lldb_dap = worktree.which("lldb-dap")?;
                Some((lldb_dap, Vec::new()))
            })
            .ok_or_else(|| "Could not find lldb-dap".to_owned())?;
        Ok(zed_extension_api::DebugAdapterBinary {
            command: Some(command),
            arguments,
            envs: config.env.into_iter().collect(),
            cwd: Some(config.cwd.unwrap_or_else(|| worktree.root_path())),
            connection: None,
            request_args: zed_extension_api::StartDebuggingRequestArguments {
                configuration,
                request,
            },
        })
    }

    fn dap_request_kind(
        &mut self,
        adapter_name: String,
        config: serde_json::Value,
    ) -> Result<zed_extension_api::StartDebuggingRequestArgumentsRequest, String> {
        if adapter_name != Self::ADAPTER_NAME {
            return Err(format!("Cannot create binary for adapter: {adapter_name}"));
        }

        match config.get("request") {
            Some(launch) if launch == "launch" => {
               Ok(zed_extension_api::StartDebuggingRequestArgumentsRequest::Launch)
            }
            Some(attach) if attach == "attach" => {
                Ok(zed_extension_api::StartDebuggingRequestArgumentsRequest::Attach)
            }
            Some(value) => Err(format!("Unexpected value for `request` key in Swift debug adapter configuration: {value:?}")),
            None => {
                Err("Missing required `request` field in Swift debug adapter configuration".into())
            }
        }
    }

    fn dap_config_to_scenario(
        &mut self,
        zed_scenario: zed_extension_api::DebugConfig,
    ) -> Result<zed_extension_api::DebugScenario, String> {
        match zed_scenario.request {
            zed_extension_api::DebugRequest::Launch(launch) => {
                let config = serde_json::to_string(&SwiftDebugConfig {
                    program: Some(launch.program),
                    env: launch.envs.into_iter().collect(),
                    cwd: launch.cwd.clone(),
                    request: "launch".to_owned(),
                    pid: None,
                    stop_on_entry: zed_scenario.stop_on_entry,
                })
                .unwrap();

                Ok(zed_extension_api::DebugScenario {
                    adapter: zed_scenario.adapter,
                    label: zed_scenario.label,
                    config,
                    tcp_connection: None,
                    build: None,
                })
            }
            zed_extension_api::DebugRequest::Attach(attach) => {
                let config = serde_json::to_string(&SwiftDebugConfig {
                    program: None,
                    env: Default::default(),
                    request: "attach".to_owned(),
                    pid: attach.process_id,
                    cwd: None,
                    stop_on_entry: zed_scenario.stop_on_entry,
                })
                .unwrap();

                Ok(zed_extension_api::DebugScenario {
                    adapter: zed_scenario.adapter,
                    label: zed_scenario.label,
                    build: None,
                    config,
                    tcp_connection: None,
                })
            }
        }
    }
}

zed::register_extension!(SwiftExtension);

#[cfg(test)]
mod runnables_test;

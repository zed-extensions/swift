mod language_server;
use language_server::SourceKitLsp;

use zed::settings::LspSettings;
use zed_extension_api::{
    self as zed,
    lsp::{Completion, Symbol},
    serde_json, CodeLabel, LanguageServerId, Result,
};

#[derive(Default)]
struct SwiftExtension {
    sourcekit_lsp: Option<SourceKitLsp>,
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
        _adapter_name: String,
        _config: zed_extension_api::DebugTaskDefinition,
        user_provider_debug_adapter_path: Option<String>,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<zed_extension_api::DebugAdapterBinary, String> {
        let command = if let Some(command) = user_provider_debug_adapter_path {
            command
        } else {
            todo!()
        };
        Ok(zed_extension_api::DebugAdapterBinary {
            command: Some(command),
            arguments: Vec::new(),
            envs: worktree.shell_env(),
            cwd: Some(worktree.root_path()),
            connection: None,
            request_args: todo!(),
        })
    }

    fn dap_request_kind(
        &mut self,
        _adapter_name: String,
        _config: serde_json::Value,
    ) -> Result<zed_extension_api::StartDebuggingRequestArgumentsRequest, String> {
        todo!()
    }

    fn dap_config_to_scenario(
        &mut self,
        _config: zed_extension_api::DebugConfig,
    ) -> Result<zed_extension_api::DebugScenario, String> {
        todo!()
    }
}

zed::register_extension!(SwiftExtension);

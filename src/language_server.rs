use zed_extension_api::lsp::{Completion, CompletionKind, Symbol, SymbolKind};
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, CodeLabel, CodeLabelSpan, EnvVars, LanguageServerId};

#[derive(Clone, Debug)]
pub struct LanguageServerBinary {
    pub path: String,
    pub args: Option<Vec<String>>,
    pub env: EnvVars,
}

#[derive(Default)]
pub struct SourceKitLsp;

impl SourceKitLsp {
    pub const SERVER_ID: &'static str = "sourcekit-lsp";

    pub fn new() -> Self {
        Self {}
    }

    fn get_executable_args() -> Vec<String> {
        Vec::new()
    }

    fn language_server_binary(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed_extension_api::Result<LanguageServerBinary> {
        let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        if let Some(binary_settings) = lsp_settings.binary {
            if let Some(path) = binary_settings.path {
                return Ok(LanguageServerBinary {
                    path,
                    args: binary_settings.arguments,
                    env: binary_settings
                        .env
                        .iter()
                        .flat_map(|env_vars| env_vars.iter())
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                });
            }
        }

        if let Some(path) = worktree.which(Self::SERVER_ID) {
            return Ok(LanguageServerBinary {
                path,
                args: Default::default(),
                env: worktree.shell_env(),
            });
        }

        Ok(LanguageServerBinary {
            path: "/usr/bin/xcrun".into(),
            args: Some(vec![Self::SERVER_ID.into()]),
            env: Default::default(),
        })
    }

    pub fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed_extension_api::Result<zed::Command> {
        let binary = self.language_server_binary(language_server_id, worktree)?;

        Ok(zed::Command {
            command: binary.path,
            args: binary.args.unwrap_or(Self::get_executable_args()),
            env: binary.env,
        })
    }

    pub fn label_for_completion(&self, completion: Completion) -> Option<CodeLabel> {
        let highlight_name = match completion.kind? {
            CompletionKind::Class | CompletionKind::Struct | CompletionKind::Enum => "type",
            CompletionKind::Function => "function",
            CompletionKind::Method => "function.method",
            CompletionKind::Variable => "variable",
            CompletionKind::Property => "property",
            _ => return None,
        };

        let label = completion.label.clone();
        Some(CodeLabel {
            code: label.clone(),
            spans: vec![CodeLabelSpan::literal(
                label,
                Some(highlight_name.to_string()),
            )],
            filter_range: (0..completion.label.len()).into(),
        })
    }

    pub fn label_for_symbol(&self, symbol: Symbol) -> Option<CodeLabel> {
        let name = &symbol.name;
        let (code, display_range) = match symbol.kind {
            SymbolKind::Function | SymbolKind::Method => {
                (format!("func {}() {{\n}}", name), 5..5 + name.len())
            }
            SymbolKind::Class => (format!("class {} {{\n}}", name), 6..6 + name.len()),
            SymbolKind::Struct => (format!("struct {} {{\n}}", name), 7..7 + name.len()),
            SymbolKind::Enum => (format!("enum {} {{\n}}", name), 5..5 + name.len()),
            SymbolKind::Variable => (format!("var {}: Type", name), 4..4 + name.len()),
            _ => return None,
        };

        Some(CodeLabel {
            code,
            spans: vec![CodeLabelSpan::code_range(display_range)],
            filter_range: (0..name.len()).into(),
        })
    }
}

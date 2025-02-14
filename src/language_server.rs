use zed_extension_api::lsp::{Completion, CompletionKind, Symbol, SymbolKind};
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, CodeLabel, CodeLabelSpan, LanguageServerId};

#[derive(Clone, Debug)]
pub struct LanguageServerBinary {
    pub path: String,
    pub args: Option<Vec<String>>,
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
                });
            }
        }

        if let Some(path) = worktree.which(Self::SERVER_ID) {
            return Ok(LanguageServerBinary {
                path,
                args: Some(Self::get_executable_args()),
            });
        }

        Ok(LanguageServerBinary {
            path: "/usr/bin/xcrun".into(),
            args: Some(vec![Self::SERVER_ID.into()]),
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
            env: worktree.shell_env(),
        })
    }

    pub fn label_for_completion(&self, completion: Completion) -> Option<CodeLabel> {
        use CompletionKind::*;

        let kind = completion.kind?;

        match kind {
            Class | Enum | Interface | Keyword | Module | Struct => {
                let highlight_name = match completion.kind? {
                    Class | Interface | Enum | Struct => Some("type".to_string()),
                    Keyword => Some("keyword".to_string()),
                    _ => None,
                };

                Some(CodeLabel {
                    code: Default::default(),
                    filter_range: (0..completion.label.len()).into(),
                    spans: vec![CodeLabelSpan::literal(completion.label, highlight_name)],
                })
            }
            EnumMember => {
                let start = "enum Enum { case ";
                let code = format!("{start}{} }}", completion.label);

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(
                        start.len()..start.len() + completion.label.len(),
                    )],
                    filter_range: (0..completion.label.find('(').unwrap_or(completion.label.len()))
                        .into(),
                })
            }
            Function => {
                let func = "func ";
                let mut return_type = String::new();

                if let Some(detail) = completion.detail {
                    if !detail.is_empty() {
                        return_type = format!(" -> {detail}");
                    }
                }

                let before_braces = format!("{func}{}{return_type}", completion.label);
                let code = format!("{before_braces} {{}}");

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(func.len()..before_braces.len())],
                    filter_range: (0..completion.label.find('(')?).into(),
                })
            }
            TypeParameter => {
                let typealias = "typealias ";
                let code = format!("{typealias}{} = {}", completion.label, completion.detail?);

                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(typealias.len()..code.len())],
                    code,
                    filter_range: (0..completion.label.len()).into(),
                })
            }
            Value => {
                let mut r#type = String::new();

                if let Some(detail) = completion.detail {
                    if !detail.is_empty() {
                        r#type = format!(": {detail}");
                    }
                }

                let var = format!("var variable{type} = ");
                let code = format!("{var}{}", completion.label);

                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(var.len()..code.len())],
                    code,
                    filter_range: (0..completion.label.len()).into(),
                })
            }
            Variable => {
                let var = "var ";
                let code = format!("{var}{}: {}", completion.label, completion.detail?);

                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(var.len()..code.len())],
                    code,
                    filter_range: (0..completion.label.len()).into(),
                })
            }
            _ => None,
        }
    }

    pub fn label_for_symbol(&self, symbol: Symbol) -> Option<CodeLabel> {
        match symbol.kind {
            SymbolKind::Method | SymbolKind::Function => {
                // Simple label: "func <name>"
                let code = format!("func {}", symbol.name);
                Some(CodeLabel {
                    code: code.clone(),
                    spans: vec![CodeLabelSpan::code_range(0..code.len())],
                    filter_range: (0..symbol.name.len()).into(),
                })
            }
            SymbolKind::Variable | SymbolKind::Constant => {
                // Simple label: "var/let <name>"
                let code = format!("var/let {}", symbol.name);
                Some(CodeLabel {
                    code: code.clone(),
                    spans: vec![CodeLabelSpan::code_range(0..code.len())],
                    filter_range: (0..symbol.name.len()).into(),
                })
            }
            SymbolKind::Class => {
                // Simple label: "class <name>"
                let code = format!("class {}", symbol.name);
                Some(CodeLabel {
                    code: code.clone(),
                    spans: vec![CodeLabelSpan::code_range(0..code.len())],
                    filter_range: (0..symbol.name.len()).into(),
                })
            }
            SymbolKind::Struct => {
                // Simple label: "struct <name>"
                let code = format!("struct {}", symbol.name);
                Some(CodeLabel {
                    code: code.clone(),
                    spans: vec![CodeLabelSpan::code_range(0..code.len())],
                    filter_range: (0..symbol.name.len()).into(),
                })
            }
            SymbolKind::Enum => {
                // Simple label: "enum <name>"
                let code = format!("enum {}", symbol.name);
                Some(CodeLabel {
                    code: code.clone(),
                    spans: vec![CodeLabelSpan::code_range(0..code.len())],
                    filter_range: (0..symbol.name.len()).into(),
                })
            }
            _ => None,
        }
    }
}

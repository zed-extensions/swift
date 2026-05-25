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

#[cfg(test)]
mod tests {
    use super::*;
    use zed_extension_api::lsp::{Symbol, SymbolKind};

    fn make_symbol(kind: SymbolKind, name: &str) -> Symbol {
        Symbol {
            kind,
            name: name.to_string(),
        }
    }

    // Asserts the three fields of a CodeLabel match the Rust built-in adapter pattern:
    // - `code` is the full valid Swift snippet (prefix + name + suffix) for tree-sitter
    // - `spans` selects prefix + name from `code` (suffix excluded from display)
    // - `filter_range` indexes into the displayed text and selects just the name,
    //   skipping the keyword prefix, so fuzzy matching works on the name alone
    fn assert_code_label(
        label: &CodeLabel,
        expected_code: &str,
        expected_display: &str,
        expected_name: &str,
    ) {
        assert_eq!(label.code, expected_code, "code field");

        assert_eq!(label.spans.len(), 1, "expected exactly one span");
        let CodeLabelSpan::CodeRange(span) = &label.spans[0] else {
            panic!("expected CodeRange span, got Literal");
        };

        let displayed = &expected_code[span.start as usize..span.end as usize];
        assert_eq!(
            displayed, expected_display,
            "span selects wrong text from code"
        );

        let filter_text =
            &displayed[label.filter_range.start as usize..label.filter_range.end as usize];
        assert_eq!(
            filter_text, expected_name,
            "filter_range should select just the symbol name"
        );
    }

    #[test]
    fn label_for_function() {
        let label = SourceKitLsp::new()
            .label_for_symbol(make_symbol(SymbolKind::Function, "myFunc"))
            .expect("expected a label for Function");
        assert_code_label(&label, "func myFunc() {}", "func myFunc", "myFunc");
    }

    #[test]
    fn label_for_method() {
        let label = SourceKitLsp::new()
            .label_for_symbol(make_symbol(SymbolKind::Method, "myMethod"))
            .expect("expected a label for Method");
        assert_code_label(&label, "func myMethod() {}", "func myMethod", "myMethod");
    }

    #[test]
    fn label_for_variable() {
        let label = SourceKitLsp::new()
            .label_for_symbol(make_symbol(SymbolKind::Variable, "myVar"))
            .expect("expected a label for Variable");
        assert_code_label(&label, "var myVar: Int", "var myVar", "myVar");
    }

    #[test]
    fn label_for_constant() {
        let label = SourceKitLsp::new()
            .label_for_symbol(make_symbol(SymbolKind::Constant, "myConst"))
            .expect("expected a label for Constant");
        assert_code_label(&label, "let myConst: Int", "let myConst", "myConst");
    }

    #[test]
    fn label_for_class() {
        let label = SourceKitLsp::new()
            .label_for_symbol(make_symbol(SymbolKind::Class, "MyClass"))
            .expect("expected a label for Class");
        assert_code_label(&label, "class MyClass {}", "class MyClass", "MyClass");
    }

    #[test]
    fn label_for_struct() {
        let label = SourceKitLsp::new()
            .label_for_symbol(make_symbol(SymbolKind::Struct, "MyStruct"))
            .expect("expected a label for Struct");
        assert_code_label(&label, "struct MyStruct {}", "struct MyStruct", "MyStruct");
    }

    #[test]
    fn label_for_enum() {
        let label = SourceKitLsp::new()
            .label_for_symbol(make_symbol(SymbolKind::Enum, "MyEnum"))
            .expect("expected a label for Enum");
        assert_code_label(&label, "enum MyEnum {}", "enum MyEnum", "MyEnum");
    }

    #[test]
    fn label_for_protocol() {
        let label = SourceKitLsp::new()
            .label_for_symbol(make_symbol(SymbolKind::Interface, "MyProtocol"))
            .expect("expected a label for Interface");
        assert_code_label(
            &label,
            "protocol MyProtocol {}",
            "protocol MyProtocol",
            "MyProtocol",
        );
    }

    #[test]
    fn label_for_property() {
        let label = SourceKitLsp::new()
            .label_for_symbol(make_symbol(SymbolKind::Property, "myProp"))
            .expect("expected a label for Property");
        assert_code_label(&label, "var myProp: Int", "var myProp", "myProp");
    }

    #[test]
    fn label_for_extension() {
        let label = SourceKitLsp::new()
            .label_for_symbol(make_symbol(SymbolKind::Namespace, "MyType"))
            .expect("expected a label for Namespace");
        assert_code_label(&label, "extension MyType {}", "extension MyType", "MyType");
    }

    #[test]
    fn label_for_enum_case() {
        let label = SourceKitLsp::new()
            .label_for_symbol(make_symbol(SymbolKind::EnumMember, "myCase"))
            .expect("expected a label for EnumMember");
        assert_code_label(&label, "enum E { case myCase }", "case myCase", "myCase");
    }

    #[test]
    fn label_for_initializer() {
        let label = SourceKitLsp::new()
            .label_for_symbol(make_symbol(SymbolKind::Constructor, "init(x:y:)"))
            .expect("expected a label for Constructor (init)");
        assert_code_label(&label, "class C { init(x:y:) }", "init(x:y:)", "init");
    }

    #[test]
    fn label_for_deinitializer() {
        let label = SourceKitLsp::new()
            .label_for_symbol(make_symbol(SymbolKind::Constructor, "deinit"))
            .expect("expected a label for Constructor (deinit)");
        assert_code_label(&label, "class C { deinit }", "deinit", "deinit");
    }

    #[test]
    fn label_for_type_parameter() {
        let label = SourceKitLsp::new()
            .label_for_symbol(make_symbol(SymbolKind::TypeParameter, "MyAlias"))
            .expect("expected a label for TypeParameter");
        assert_code_label(
            &label,
            "typealias MyAlias = Any",
            "typealias MyAlias",
            "MyAlias",
        );
    }

    #[test]
    fn label_for_unhandled_kind_returns_none() {
        let label = SourceKitLsp::new().label_for_symbol(make_symbol(SymbolKind::File, "anything"));
        assert!(label.is_none(), "expected None for unhandled SymbolKind");
    }
}

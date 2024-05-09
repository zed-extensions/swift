use zed_extension_api::{self as zed, Result};

struct SwiftExtension {}

impl zed::Extension for SwiftExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: "/usr/bin/xcrun".into(),
            args: vec!["sourcekit-lsp".into()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(SwiftExtension);

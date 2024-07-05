use zed_extension_api::{self as zed, Result};

struct SwiftExtension {}

impl zed::Extension for SwiftExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if let Some(path) = worktree.which("sourcekit-lsp") {
            return Ok(zed::Command {
                command: path,
                args: Default::default(),
                env: worktree.shell_env(),
            });
        }

        Ok(zed::Command {
            command: "/usr/bin/xcrun".into(),
            args: vec!["sourcekit-lsp".into()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(SwiftExtension);

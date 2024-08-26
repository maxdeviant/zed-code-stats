use zed_extension_api::{
    register_extension, Command, Extension, LanguageServerId, Result, Worktree,
};

struct CodeStatsExtension {}

impl Extension for CodeStatsExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let command = worktree.which("code-stats-ls").ok_or_else(|| {
            "code-stats-ls must be installed manually from https://github.com/maxdeviant/code-stats-ls".to_string()
        })?;

        Ok(Command {
            command,
            args: Vec::new(),
            env: worktree.shell_env(),
        })
    }
}

register_extension!(CodeStatsExtension);

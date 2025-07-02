use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result, settings::LspSettings};

const TYPESPEC_LANGUAGE_SERVER_NAME: &str = "tsp-server";

struct TypespecLanguageServerBinary {
    path: String,
    args: Option<Vec<String>>,
}

struct TypespecExtension;

impl TypespecExtension {
    fn language_server_binary(
        &self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<TypespecLanguageServerBinary> {
        let binary_settings = LspSettings::for_worktree(TYPESPEC_LANGUAGE_SERVER_NAME, worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(TypespecLanguageServerBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = worktree.which(TYPESPEC_LANGUAGE_SERVER_NAME) {
            return Ok(TypespecLanguageServerBinary {
                path,
                args: binary_args,
            });
        }

        Err(format!("{TYPESPEC_LANGUAGE_SERVER_NAME} not found in PATH",))
    }
}

impl zed::Extension for TypespecExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let typespec_binary = self.language_server_binary(language_server_id, worktree)?;
        let argument_path: String = typespec_binary.path.clone();
        let arguments: Vec<String> = vec![argument_path.into(), "--stdio".into()];
        Ok(zed::Command {
            command: typespec_binary.path,
            args: typespec_binary.args.unwrap_or_else(|| arguments),
            env: Default::default(),
        })
    }
}

zed::register_extension!(TypespecExtension);

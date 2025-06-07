use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{self as zed, serde_json, Command, ContextServerId, Project, Result};

const PACKAGE_NAME: &str = "byterover-mcp";
const SERVER_PATH: &str = "node_modules/byterover-mcp/dist/cli.js";

struct ByteRoverMcpExtension;

#[derive(Debug, Deserialize, JsonSchema)]
struct ByteRoverContextServerSettings {
    memory_workspace_key: String,
    user_email: String,
}

impl zed::Extension for ByteRoverMcpExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let latest_version = zed::npm_package_latest_version(PACKAGE_NAME)?;
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(latest_version.as_ref()) {
            zed::npm_install_package(PACKAGE_NAME, &latest_version)?;
        }

        let settings = ContextServerSettings::for_project("mcp-server-byterover", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `memory_workspace_key` or `user_email` settings".into());
        };
        let settings: ByteRoverContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(SERVER_PATH)
                    .to_string_lossy()
                    .to_string(),
                format!(
                    "--byterover-public-api-key={}",
                    settings.memory_workspace_key
                ),
                format!("--user-id={}", settings.user_email),
                "--stdio".to_string(),
            ],
            env: vec![],
        })
    }
}

zed::register_extension!(ByteRoverMcpExtension);

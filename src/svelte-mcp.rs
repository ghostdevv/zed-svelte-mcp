use zed_extension_api::{self as zed, Result};
use std::env;

const MCP_SERVER_PACKAGE_NAME: &str = "@sveltejs/mcp";

struct SvelteMCPExtension {}

impl SvelteMCPExtension {
    fn install_or_update_mcp(&self) -> Result<()> {
        let installed_version = zed::npm_package_installed_version(MCP_SERVER_PACKAGE_NAME)?;
        let latest_version = zed::npm_package_latest_version(MCP_SERVER_PACKAGE_NAME)?;

        if installed_version.is_none() || installed_version.unwrap() != latest_version {
            zed::npm_install_package(MCP_SERVER_PACKAGE_NAME, &latest_version)?;
        }

        Ok(())
    }
}

impl zed::Extension for SvelteMCPExtension {
    fn new() -> Self {
        Self {}
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &zed_extension_api::ContextServerId,
        _project: &zed_extension_api::Project,
    ) -> Result<zed::Command> {
        self.install_or_update_mcp()?;

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .map_err(|err| err.to_string())?
                    .join("node_modules")
                    .join(MCP_SERVER_PACKAGE_NAME)
                    .join("dist/index.mjs")
                    .to_string_lossy()
                    .to_string()
            ],
            env: Default::default(),
        })
    }
}

zed::register_extension!(SvelteMCPExtension);

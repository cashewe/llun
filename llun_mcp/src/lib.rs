use figment::{
    Figment,
    providers::{Format, Toml},
};
use serde::{Deserialize, Serialize};
use rmcp::{
    model::*,
    service::RequestContext,
    tool, tool_handler, tool_router,
    RoleServer, ServerHandler,
};
use rmcp::model::ErrorData as McpError;
use tracing::{info, debug, error};

use llun_core::data::DEFAULT_CONFIG;
use llun_core::rules::RuleManager;


/// Args we want to pull from the users (or our default) toml file.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct RulesArgs {
    /// rules to utilise (overrides default values)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub select: Vec<String>,

    /// rules to add to the defaults
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extend_select: Vec<String>,

    /// rules to ignore from the default list
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ignore: Vec<String>,
}

/// MCP Server
#[derive(Clone)]
pub struct LlunServer {
    tool_router: rmcp::handler::server::router::tool::ToolRouter<LlunServer>
}

/// dispatch all 'tool's to our mcp server
#[tool_router]
impl LlunServer {
    pub fn new() -> Self {
        Self { tool_router: Self::tool_router() } // tool_router comes from #[tool_router]
    }

    /// MCP 'tool' for accessing the users selected rules and returning them to the agent
    #[tool(description="Get a user defined selection of architectural rules, patterns and principles that should be followed when building new solutions. Call this tool prior to beginning coding or design tasks in order to fully understand the required context for the users specification.")]
    async fn get_rules(&self) -> Result<CallToolResult, McpError> {
        // assume user runs this from root... im not sure how else to do it really?
        let config: RulesArgs = Figment::new()
            .merge(Toml::string(DEFAULT_CONFIG)) // default values are set in the data file in the library
            .merge(Toml::file("pyproject.toml").nested())
            .merge(Toml::file("llun.toml"))
            .select("tool.llun")
            .extract()
            .map_err(|e| {
                error!("Failed to load config: {}", e);
                McpError::internal_error(
                    format!("Failed to load configuration: {}", e),
                    None,
                )
            })?;
        debug!("Read user arguments from tomls...");

        // i have to map loads of errors here as the impl requires errors of a certain type.
        // makes it look like way more code that it really is - its just pulling rules from our lib in actuality
        let rule_manager = RuleManager::new().map_err(|e| {
            error!("Failed to create RuleManager: {}", e);
            McpError::internal_error(format!("Failed to initialize rules: {}", e), None)
        })?;
        let rules = rule_manager
            .load_from_cli(config.select, config.extend_select, config.ignore)
            .map_err(|e| {
                error!("Failed to load rules: {}", e);
                McpError::internal_error(format!("Failed to load rules: {}", e), None)
            })?;
        let formatted_rules = format!("{}", rules); // should these be formated in other more sexy ways?

        Ok(CallToolResult::success(vec![Content::text(
            formatted_rules,
        )]))
    }
}

/// provide the MCP server with the relevant metadata
#[tool_handler]
impl ServerHandler for LlunServer {
    /// sent to clients when they connect to the llun server
    fn get_info(&self) -> ServerInfo {
        ServerInfo{
            protocol_version: ProtocolVersion::LATEST,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("Server which uses the popular 'Llun' tool to provide user defined architectural rules which must be followed when engineering new solutions.".to_string())
        }
    }

    /// lift and shift from example repo - pretty sure its boilerplate
    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        info!("Client connected");
        Ok(self.get_info())
    }
}
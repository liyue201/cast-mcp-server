use rmcp::{ServerHandler, handler::server::router::tool::ToolRouter, model::*, tool_handler};

#[derive(Clone)]
pub struct Server {
    tool_router: ToolRouter<Self>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            tool_router: Server::utility_router() + Server::block_router(),
        }
    }
}

#[tool_handler]
impl ServerHandler for Server {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A MCP server for cast".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

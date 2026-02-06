#![allow(dead_code)]

use std::{any::Any, sync::Arc};

use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{
        router::{prompt::PromptRouter, tool::ToolRouter},
        wrapper::Parameters,
    },
    model::*,
    prompt, prompt_handler, prompt_router, schemars,
    service::RequestContext,
    task_handler,
    task_manager::{
        OperationDescriptor, OperationMessage, OperationProcessor, OperationResultTransport,
    },
    tool, tool_handler, tool_router,
};
use serde_json::json;
use tokio::sync::Mutex;
use tracing::info;

#[derive(Clone)]
pub struct Server {
    tool_router: ToolRouter<Server>,
}

#[tool_router]
impl Server {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
    #[tool(description = "ping ")]
    async fn ping(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text("pong")]))
    }
}

#[tool_handler]
impl ServerHandler for Server {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A simple calculator".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

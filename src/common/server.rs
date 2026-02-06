#![allow(dead_code)]
#![allow(unused)]
use std::{any::Any, sync::Arc};
use rand::random;
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

use cast::SimpleCast;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct Request {
    pub int_type: String,
}

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
    #[tool(description = "a test tool")]
    async fn ping(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text("pong")]))
    }

    #[tool(
    description = r#"
    Description: Get maximum value for integer type.
    Parameters:
        int_type: a string representing the integer type. Possible values are int8, uint8, int16, uint16, int32, uint32, int64, uint64, int256, uint256.
    "#
    )]
    async fn max_int(&self, Parameters(req): Parameters<Request>) -> Result<CallToolResult, McpError> {
        let res = SimpleCast::max_int(&req.int_type).map_err(|e| {
            tracing::error!("Failed to get max int: {}", e);
            McpError::new(ErrorCode::INTERNAL_ERROR, "Failed to compute max int",  None)
        })?;
        Ok(CallToolResult::success(vec![Content::text(res)]))
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

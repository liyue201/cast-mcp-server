#![allow(dead_code)]
#![allow(unused)]
use std::{any::Any, sync::Arc};

use cast::SimpleCast;
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
use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use serde_json::json;
use tokio::sync::Mutex;
use tracing::info;

use crate::common::server::Server;

#[tool_router(router = block_router, vis = "pub")]
impl Server {
    #[tool(description = "a test tool for block")]
    async fn block(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text("block")]))
    }
}

#![allow(dead_code)]
#![allow(unused)]
use std::{any::Any, sync::Arc};

use alloy_primitives::{Address, B256, eip191_hash_message, hex, keccak256};
use cast::SimpleCast;
use rand::random;
use rmcp::{
    ErrorData, RoleServer, ServerHandler,
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

fn default_int() -> String {
    "int256".to_string()
}

fn default_uint() -> String {
    "uint256".to_string()
}

#[derive(Debug, serde::Deserialize, DefaultFromSerde, schemars::JsonSchema)]
pub struct MaxIntArgs {
    #[serde(default = "default_int")]
    pub r#type: String,
}

#[derive(Debug, serde::Deserialize, DefaultFromSerde, schemars::JsonSchema)]
pub struct MaxUIntArgs {
    #[serde(default = "default_uint")]
    pub r#type: String,
}

#[tool_router(router = utility_router, vis = "pub")]
impl Server {
    #[tool(description = "A test tool")]
    async fn ping(&self) -> Result<CallToolResult, ErrorData> {
        Ok(CallToolResult::success(vec![Content::text("pong")]))
    }

    #[tool(description = r#"
    Get maximum value for integer type.
    Parameters:
        type: a string representing the integer type. Possible values are int8, int16, int32, int64, int256.
    "#)]
    async fn max_int(
        &self,
        Parameters(MaxIntArgs { r#type: t }): Parameters<MaxIntArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let res = SimpleCast::max_int(&t).map_err(|e| {
            tracing::error!("Failed to get max int: {}", e);
            ErrorData::invalid_params("Failed to get max int", None)
        })?;
        Ok(CallToolResult::success(vec![Content::text(res)]))
    }

    #[tool(description = r#"
    Get minimum value for integer type.
    Parameters:
        type: a string representing the integer type. Possible values are int8, int16, int32, int64, int256.
    "#)]
    async fn min_int(
        &self,
        Parameters(MaxIntArgs { r#type: t }): Parameters<MaxIntArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let res = SimpleCast::min_int(&t).map_err(|e| {
            tracing::error!("Failed to get min int: {}", e);
            ErrorData::invalid_params("Failed to get min int", None)
        })?;
        Ok(CallToolResult::success(vec![Content::text(res)]))
    }

    #[tool(description = r#"
    Get maximum value for unsigned integer type.
    Parameters:
        type: a string representing the unsigned integer type. Possible values are uint8, uint16, uint32, uint64, uint256.
    "#)]
    async fn max_uint(
        &self,
        Parameters(MaxUIntArgs { r#type: t }): Parameters<MaxUIntArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let res = SimpleCast::max_int(&t).map_err(|e| {
            tracing::error!("Failed to get max unsigned int: {}", e);
            ErrorData::invalid_params("Failed to get max unsigned int", None)
        })?;
        Ok(CallToolResult::success(vec![Content::text(res)]))
    }

    #[tool(description = "Get the zero address")]
    async fn address_zero(&self) -> Result<CallToolResult, ErrorData> {
        Ok(CallToolResult::success(vec![Content::text(
            Address::ZERO.to_string(),
        )]))
    }

    #[tool(description = "Get the zero hash")]
    async fn hash_zero(&self) -> Result<CallToolResult, ErrorData> {
        Ok(CallToolResult::success(vec![Content::text(
            B256::ZERO.to_string(),
        )]))
    }
}

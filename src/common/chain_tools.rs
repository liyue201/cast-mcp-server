#![allow(dead_code)]
#![allow(unused)]
use core::convert;
use std::{any::Any, sync::Arc};

use cast::{Cast, SimpleCast};
use foundry_cli::{
    opts::{EtherscanOpts, GlobalArgs, RpcOpts},
    utils,
    utils::LoadConfig,
};
use alloy_provider::Provider;
use foundry_config::{Chain, Config, error, error::ExtractConfigError};
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
use serde_json::{Value, json};
use tokio::sync::Mutex;
use tracing::info;

use crate::common::{server::Server, utility_tools::MaxIntArgs};

fn default_rpc() -> String {
    "http://localhost:8545".to_string()
}

#[derive(Debug, serde::Deserialize, DefaultFromSerde, schemars::JsonSchema)]
pub struct ChainArgs {
    #[serde(default = "default_rpc")]
    pub rpc: String,
}

#[tool_router(router = chain_router, vis = "pub")]
impl Server {
    #[tool(description = "
      Get the symbolic name of the current chain
      Parameters:
        rpc: The RPC endpoint, default value is http://localhost:8545.
    ")]
    async fn chain(
        &self,
        Parameters(ChainArgs { rpc: rpc_url }): Parameters<ChainArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let rpc = RpcOpts {
            url: Some(rpc_url),
            accept_invalid_certs: false,
            no_proxy: false,
            flashbots: false,
            jwt_secret: None,
            rpc_timeout: None,
            rpc_headers: None,
            curl: false,
        };
        let config = rpc.load_config().map_err(|e| {
            ErrorData::parse_error("Invalid RPC URL", Some(Value::String(e.to_string())))
        })?;

        let provider = utils::get_provider(&config).map_err(|e| {
            ErrorData::internal_error("Failed to get provider", Some(Value::String(e.to_string())))
        })?;

        let cli = Cast::new(provider);
        let chain = cli.chain().await.map_err(|e| {
            ErrorData::internal_error("Failed to get chain", Some(Value::String(e.to_string())))
        })?;

        Ok(CallToolResult::success(vec![Content::text(chain)]))
    }

    #[tool(description = "
      Get the chain ID of the current chain
      Parameters:
        rpc: The RPC endpoint, default value is http://localhost:8545.
    ")]
    async fn chain_id(
        &self,
        Parameters(ChainArgs { rpc: rpc_url }): Parameters<ChainArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let rpc = RpcOpts {
            url: Some(rpc_url),
            accept_invalid_certs: false,
            no_proxy: false,
            flashbots: false,
            jwt_secret: None,
            rpc_timeout: None,
            rpc_headers: None,
            curl: false,
        };
        let config = rpc.load_config().map_err(|e| {
            ErrorData::parse_error("Invalid RPC URL", Some(Value::String(e.to_string())))
        })?;

        let provider = utils::get_provider(&config).map_err(|e| {
            ErrorData::internal_error("Failed to get provider", Some(Value::String(e.to_string())))
        })?;

        let cli = Cast::new(provider);
        let chain_id = cli.chain_id().await.map_err(|e| {
            ErrorData::internal_error("Failed to get chain", Some(Value::String(e.to_string())))
        })?;

        Ok(CallToolResult::success(vec![Content::text(chain_id.to_string())]))
    }

    #[tool(description = "
      Get the current client version.
      Parameters:
        rpc: The RPC endpoint, default value is http://localhost:8545.
    ")]
    async fn client(
        &self,
        Parameters(ChainArgs { rpc: rpc_url }): Parameters<ChainArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let rpc = RpcOpts {
            url: Some(rpc_url),
            accept_invalid_certs: false,
            no_proxy: false,
            flashbots: false,
            jwt_secret: None,
            rpc_timeout: None,
            rpc_headers: None,
            curl: false,
        };
        let config = rpc.load_config().map_err(|e| {
            ErrorData::parse_error("Invalid RPC URL", Some(Value::String(e.to_string())))
        })?;

        let provider = utils::get_provider(&config).map_err(|e| {
            ErrorData::internal_error("Failed to get provider", Some(Value::String(e.to_string())))
        })?;

        let version = provider.get_client_version().await.map_err(|e| {
            ErrorData::internal_error("Failed to get client version", Some(Value::String(e.to_string())))
        })?;

        Ok(CallToolResult::success(vec![Content::text(version)]))
    }
}

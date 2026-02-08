use cast::Cast;
use foundry_cli::{opts::RpcOpts, utils, utils::LoadConfig};
use rmcp::{
    ErrorData, handler::server::wrapper::Parameters, model::*, schemars, tool, tool_router,
};
use serde_default::DefaultFromSerde;
use serde_json::Value;

use crate::common::{common::*, server::Server};

#[derive(Debug, serde::Deserialize, DefaultFromSerde, schemars::JsonSchema)]
pub struct AgeArgs {
    #[serde(default = "default_rpc")]
    pub rpc: String,
    #[serde(default)]
    block: Option<String>,
}

#[tool_router(router = block_router, vis = "pub")]
impl Server {
    #[tool(description = "
      Get the timestamp of a block.
      Parameters:
        rpc: The RPC endpoint, default value is http://localhost:8545.
        block: The block height to query at. Can also be the tags earliest, finalized, safe, latest, pending or block hash.
    ")]
    async fn age(
        &self,
        Parameters(args): Parameters<AgeArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let rpc = RpcOpts {
            url: Some(args.rpc.clone()),
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

        let age = Cast::new(provider)
            .age(get_block_id(args.block))
            .await
            .map_err(|e| {
                ErrorData::internal_error(
                    "Failed to get block age",
                    Some(Value::String(e.to_string())),
                )
            })?;

        Ok(CallToolResult::success(vec![Content::text(age)]))
    }
}

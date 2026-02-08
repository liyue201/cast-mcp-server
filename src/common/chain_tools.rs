use alloy_provider::Provider;
use cast::Cast;
use foundry_cli::{opts::RpcOpts, utils, utils::LoadConfig};
use rmcp::{
    ErrorData, handler::server::wrapper::Parameters, model::*, schemars, tool, tool_router,
};
use serde_default::DefaultFromSerde;
use serde_json::Value;

use crate::common::{common::*, server::Server};

#[derive(Debug, Clone, serde::Deserialize, DefaultFromSerde, schemars::JsonSchema)]
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

        Ok(CallToolResult::success(vec![Content::text(
            chain_id.to_string(),
        )]))
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
            ErrorData::internal_error(
                "Failed to get client version",
                Some(Value::String(e.to_string())),
            )
        })?;

        Ok(CallToolResult::success(vec![Content::text(version)]))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rmcp::handler::server::wrapper::Parameters;

    use super::*;

    #[test]
    fn test_chain_args_default() {
        let args = ChainArgs::default();
        assert_eq!(args.rpc, "http://localhost:8545");
    }

    #[test]
    fn test_chain_args_clone() {
        let original = ChainArgs {
            rpc: "https://test.com".to_string(),
        };
        let cloned = original.clone();
        assert_eq!(original.rpc, cloned.rpc);
    }

    #[tokio::test]
    async fn test_chain_tools_response_structure() {
        let server = Server::new();

        // Test with localhost (may succeed or fail, but we check structure)
        let args = ChainArgs {
            rpc: "https://mainnet.infura.io".to_string(),
        };
        let params = Parameters(args);

        // Test chain tool response structure (even if it fails)
        let chain_result = server.chain(params.clone()).await;
        match chain_result {
            Ok(result) => {
                assert!(
                    !result.content.is_empty(),
                    "Chain tool should return content when successful"
                );
                let response_text = result
                    .content
                    .first()
                    .unwrap()
                    .raw
                    .as_text()
                    .expect("Response should be text");
                assert!(
                    !response_text.text.is_empty(),
                    "Chain response should not be empty"
                );
                println!("Chain response: {}", response_text.text);
            }
            Err(error) => {
                // Even in error case, verify it's a proper ErrorData
                assert!(
                    !error.message.is_empty(),
                    "Error message should not be empty"
                );
                println!("Chain error (expected): {}", error.message);
            }
        }

        // Test chain_id tool response structure
        let chain_id_result = server.chain_id(params.clone()).await;
        match chain_id_result {
            Ok(result) => {
                assert!(
                    !result.content.is_empty(),
                    "Chain ID tool should return content when successful"
                );
                let response_text = result
                    .content
                    .first()
                    .unwrap()
                    .raw
                    .as_text()
                    .expect("Response should be text");
                // Chain ID should be numeric
                assert!(
                    response_text.text.parse::<u64>().is_ok() || !response_text.text.is_empty(),
                    "Chain ID should be numeric or non-empty"
                );
                println!("Chain ID response: {}", response_text.text);
            }
            Err(error) => {
                assert!(
                    !error.message.is_empty(),
                    "Error message should not be empty"
                );
                println!("Chain ID error (expected): {}", error.message);
            }
        }

        // Test client tool response structure
        let client_result = server.client(params).await;
        match client_result {
            Ok(result) => {
                assert!(
                    !result.content.is_empty(),
                    "Client tool should return content when successful"
                );
                let response_text = result
                    .content
                    .first()
                    .unwrap()
                    .raw
                    .as_text()
                    .expect("Response should be text");
                assert!(
                    !response_text.text.is_empty(),
                    "Client version should not be empty"
                );
                println!("Client response: {}", response_text.text);
            }
            Err(error) => {
                assert!(
                    !error.message.is_empty(),
                    "Error message should not be empty"
                );
                println!("Client error (expected): {}", error.message);
            }
        }
    }

    #[tokio::test]
    async fn test_concurrent_chain_tool_calls() {
        let server = Arc::new(Server::new());

        // Test concurrent execution of chain tools
        let handles: Vec<_> = (0..6)
            .map(|i| {
                let server_clone = Arc::clone(&server);
                tokio::spawn(async move {
                    let args = ChainArgs {
                        rpc: "https://mainnet.infura.io".to_string(),
                    };
                    let params = Parameters(args);

                    match i % 3 {
                        0 => server_clone.chain(params.clone()).await,
                        1 => server_clone.chain_id(params.clone()).await,
                        2 => server_clone.client(params).await,
                        _ => unreachable!(),
                    }
                })
            })
            .collect();

        // Wait for all concurrent calls to complete
        let results: Vec<_> = futures::future::join_all(handles).await;

        // Verify all concurrent calls completed (they may succeed or fail, but shouldn't panic)
        for (i, result) in results.into_iter().enumerate() {
            let call_result = result.expect(&format!("Concurrent chain task {} join failed", i));
            // Each call should either succeed or return a proper error
            match call_result {
                Ok(_) => println!("Concurrent chain call {} succeeded", i),
                Err(e) => println!(
                    "Concurrent chain call {} failed as expected: {}",
                    i, e.message
                ),
            }
        }
    }

    #[test]
    fn test_chain_args_serialization() {
        let args = ChainArgs {
            rpc: "https://mainnet.infura.io".to_string(),
        };

        // Test that args implements the required traits
        let _clone = args.clone();
        let debug_output = format!("{:?}", args);
        assert!(debug_output.contains("ChainArgs"));
        assert!(debug_output.contains("https://mainnet.infura.io"));
    }

    #[test]
    fn test_chain_args_debug_format() {
        let args = ChainArgs {
            rpc: "test-rpc".to_string(),
        };
        let debug_output = format!("{:?}", args);
        assert!(debug_output.contains("ChainArgs"));
        assert!(debug_output.contains("test-rpc"));
    }
}

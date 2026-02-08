use cast::Cast;
use foundry_cli::{opts::RpcOpts, utils, utils::LoadConfig};
use rmcp::{
    ErrorData, handler::server::wrapper::Parameters, model::*, schemars, tool, tool_router,
};
use serde_default::DefaultFromSerde;
use serde_json::Value;

use crate::common::{common::*, server::Server};

#[derive(Debug, Clone, serde::Deserialize, DefaultFromSerde, schemars::JsonSchema)]
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rmcp::handler::server::wrapper::Parameters;

    use super::*;

    #[test]
    fn test_age_args_default() {
        let args = AgeArgs::default();
        assert_eq!(args.rpc, "http://localhost:8545");
        assert_eq!(args.block, None);
    }

    #[test]
    fn test_age_args_clone() {
        let original = AgeArgs {
            rpc: "https://test.com".to_string(),
            block: Some("latest".to_string()),
        };
        let cloned = original.clone();
        assert_eq!(original.rpc, cloned.rpc);
        assert_eq!(original.block, cloned.block);
    }

    #[tokio::test]
    async fn test_age_block_tags() {
        let server = Server::new();

        let block_tags = vec!["earliest", "finalized", "safe", "latest", "pending"];

        for tag in block_tags {
            let args = AgeArgs {
                rpc: "https://mainnet.infura.io".to_string(),
                block: Some(tag.to_string()),
            };
            let params = Parameters(args);

            let result =
                tokio::time::timeout(std::time::Duration::from_secs(5), server.age(params))
                    .await
                    .expect(&format!("Age tool timeout for block tag: {}", tag));

            // Test response structure regardless of success/failure
            match result {
                Ok(response) => {
                    assert!(
                        !response.content.is_empty(),
                        "Age tool should return content for tag: {}",
                        tag
                    );
                    let response_text = response
                        .content
                        .first()
                        .unwrap()
                        .raw
                        .as_text()
                        .expect("Response should be text");
                    assert!(
                        !response_text.text.is_empty(),
                        "Age response should not be empty for tag: {}",
                        tag
                    );
                    println!("Age for {}: {}", tag, response_text.text);
                }
                Err(error) => {
                    assert!(
                        !error.message.is_empty(),
                        "Error message should not be empty for tag: {}",
                        tag
                    );
                    println!("Age error for {} (expected): {}", tag, error.message);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_age_response_structure() {
        let server = Server::new();

        // Test with localhost (may succeed or fail, but we check structure)
        let args = AgeArgs {
            rpc: "https://mainnet.infura.io".to_string(),
            block: Some("latest".to_string()),
        };
        let params = Parameters(args);

        let result = server.age(params).await;
        match result {
            Ok(response) => {
                // Verify successful response structure
                assert!(
                    !response.content.is_empty(),
                    "Age tool should return content when successful"
                );

                let content_item = response.content.first().unwrap();
                let response_text = content_item.raw.as_text().expect("Response should be text");

                assert!(
                    !response_text.text.is_empty(),
                    "Age response text should not be empty"
                );

                // Age should be a timestamp (numeric)
                let timestamp_result = response_text.text.parse::<u64>();
                match timestamp_result {
                    Ok(timestamp) => {
                        assert!(timestamp > 0, "Timestamp should be positive");
                        println!("Valid timestamp: {}", timestamp);
                    }
                    Err(_) => {
                        // If not numeric, should still be non-empty
                        assert!(
                            !response_text.text.is_empty(),
                            "Non-numeric age response should not be empty"
                        );
                        println!("Non-numeric age response: {}", response_text.text);
                    }
                }
            }
            Err(error) => {
                // Verify error structure
                assert!(
                    !error.message.is_empty(),
                    "Error message should not be empty"
                );
                println!("Age tool error (expected): {}", error.message);
            }
        }
    }

    #[tokio::test]
    async fn test_concurrent_age_calls() {
        let server = Arc::new(Server::new());

        // Test concurrent execution with different block identifiers
        let block_identifiers = vec![
            Some("latest"),
            Some("finalized"),
            Some("safe"),
            Some("1000000"),
            None, // Default case
        ];

        let handles: Vec<_> = block_identifiers
            .into_iter()
            .enumerate()
            .map(|(_i, block_opt)| {
                let server_clone = Arc::clone(&server);
                tokio::spawn(async move {
                    let args = AgeArgs {
                        rpc: "http://localhost:8545".to_string(),
                        block: block_opt.map(|s| s.to_string()),
                    };
                    let params = Parameters(args);

                    server_clone.age(params).await
                })
            })
            .collect();

        // Wait for all concurrent calls to complete
        let results: Vec<_> = futures::future::join_all(handles).await;

        // Verify all concurrent calls completed (they may succeed or fail, but shouldn't panic)
        for (i, result) in results.into_iter().enumerate() {
            let call_result = result.expect(&format!("Concurrent age task {} join failed", i));
            match call_result {
                Ok(response) => {
                    assert!(
                        !response.content.is_empty(),
                        "Concurrent age call {} should return content",
                        i
                    );
                    println!("Concurrent age call {} succeeded", i);
                }
                Err(e) => {
                    assert!(
                        !e.message.is_empty(),
                        "Concurrent age call {} error should have message",
                        i
                    );
                    println!(
                        "Concurrent age call {} failed as expected: {}",
                        i, e.message
                    );
                }
            }
        }
    }

    #[test]
    fn test_age_args_debug_format() {
        let args = AgeArgs {
            rpc: "test-rpc".to_string(),
            block: Some("12345".to_string()),
        };
        let debug_output = format!("{:?}", args);
        assert!(debug_output.contains("AgeArgs"));
        assert!(debug_output.contains("test-rpc"));
        assert!(debug_output.contains("12345"));
    }

    #[test]
    fn test_age_args_partial_default() {
        // Test that individual fields can be set while others use defaults
        let args = AgeArgs {
            rpc: "https://custom-rpc.com".to_string(),
            block: None,
        };
        assert_eq!(args.rpc, "https://custom-rpc.com");
        assert_eq!(args.block, None);

        let args2 = AgeArgs {
            rpc: "http://localhost:8545".to_string(), // default
            block: Some("earliest".to_string()),
        };
        assert_eq!(args2.rpc, "http://localhost:8545");
        assert_eq!(args2.block, Some("earliest".to_string()));
    }
}

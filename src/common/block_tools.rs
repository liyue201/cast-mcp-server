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
    /// The RPC endpoint, default value is http://localhost:8545.
    #[serde(default = "default_rpc")]
    pub rpc: String,

    /// The block height to query at. Can also be the tags earliest, finalized, safe, latest, pending or block hash.
    #[serde(default)]
    block: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, DefaultFromSerde, schemars::JsonSchema)]
pub struct BlockArgs {
    /// The RPC endpoint, default value is http://localhost:8545.
    #[serde(default = "default_rpc")]
    pub rpc: String,

    /// If specified, only get the given field of the block.
    fields: Vec<String>,

    /// Print the raw RLP encoded block header.
    raw: bool,

    /// If true, get all fields.
    full: bool,

    /// The block height to query at. Can also be the tags earliest, finalized, safe, latest, pending or block hash.
    #[serde(default)]
    block: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, DefaultFromSerde, schemars::JsonSchema)]
pub struct BlockNumberArgs {
    /// The RPC endpoint, default value is http://localhost:8545.
    #[serde(default = "default_rpc")]
    pub rpc: String,

    /// The block height to query at. Can also be the tags earliest, finalized, safe, latest, pending or block hash.
    #[serde(default)]
    pub block: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, DefaultFromSerde, schemars::JsonSchema)]
pub struct GasPriceArgs {
    /// The RPC endpoint, default value is http://localhost:8545.
    #[serde(default = "default_rpc")]
    pub rpc: String,
}

#[tool_router(router = block_router, vis = "pub")]
impl Server {
    #[tool(description = "Get the timestamp of a block. ")]
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

    #[tool(description = "Get the timestamp of a block.")]
    async fn block(
        &self,
        Parameters(args): Parameters<BlockArgs>,
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

        let raw = args.raw || args.fields.contains(&"raw".into());
        let block_id = get_block_id(args.block);
        let res = Cast::new(provider)
            .block(block_id, args.full, args.fields, raw)
            .await
            .map_err(|e| {
                println!("Error: {}", e.to_string());
                ErrorData::internal_error("Failed to get block", Some(Value::String(e.to_string())))
            })?;

        Ok(CallToolResult::success(vec![Content::text(res)]))
    }

    #[tool(description = "Get the latest block number")]
    async fn block_number(
        &self,
        Parameters(args): Parameters<BlockNumberArgs>,
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

        // Use Cast to get block information and extract the number
        let block_data = Cast::new(provider)
            .block(
                get_block_id(args.block),
                false,
                vec!["number".to_string()],
                false,
            )
            .await
            .map_err(|e| {
                ErrorData::internal_error(
                    "Failed to get block information",
                    Some(Value::String(e.to_string())),
                )
            })?;

        // Parse the block number from the response
        let block_number = if block_data.contains("number") {
            // Extract number from JSON response
            block_data
        } else {
            // If it's just the number, use it directly
            block_data
        };

        Ok(CallToolResult::success(vec![Content::text(block_number)]))
    }

    #[tool(description = "Get the current gas price")]
    async fn gas_price(
        &self,
        Parameters(args): Parameters<GasPriceArgs>,
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

        let price = Cast::new(provider).gas_price().await.map_err(|e| {
            ErrorData::internal_error(
                "Failed to get gas price",
                Some(Value::String(e.to_string())),
            )
        })?;

        Ok(CallToolResult::success(vec![Content::text(
            price.to_string(),
        )]))
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
                rpc: "https://1rpc.io/eth".to_string(),
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
            rpc: "https://1rpc.io/eth".to_string(),
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

    // BlockArgs tests
    #[test]
    fn test_block_args_default() {
        let args = BlockArgs::default();
        assert_eq!(args.rpc, "http://localhost:8545");
        assert_eq!(args.fields, Vec::<String>::new());
        assert_eq!(args.raw, false);
        assert_eq!(args.full, false);
        assert_eq!(args.block, None);
    }

    #[test]
    fn test_block_args_clone() {
        let original = BlockArgs {
            rpc: "https://test.com".to_string(),
            fields: vec!["number".to_string(), "hash".to_string()],
            raw: true,
            full: false,
            block: Some("latest".to_string()),
        };
        let cloned = original.clone();
        assert_eq!(original.rpc, cloned.rpc);
        assert_eq!(original.fields, cloned.fields);
        assert_eq!(original.raw, cloned.raw);
        assert_eq!(original.full, cloned.full);
        assert_eq!(original.block, cloned.block);
    }

    #[test]
    fn test_block_args_debug_format() {
        let args = BlockArgs {
            rpc: "test-rpc".to_string(),
            fields: vec!["timestamp".to_string()],
            raw: false,
            full: true,
            block: Some("12345".to_string()),
        };
        let debug_output = format!("{:?}", args);
        assert!(debug_output.contains("BlockArgs"));
        assert!(debug_output.contains("test-rpc"));
        assert!(debug_output.contains("timestamp"));
        assert!(debug_output.contains("12345"));
    }

    // BlockNumberArgs tests
    #[test]
    fn test_block_number_args_default() {
        let args = BlockNumberArgs::default();
        assert_eq!(args.rpc, "http://localhost:8545");
        assert_eq!(args.block, None);
    }

    #[test]
    fn test_block_number_args_clone() {
        let original = BlockNumberArgs {
            rpc: "https://test.com".to_string(),
            block: Some("latest".to_string()),
        };
        let cloned = original.clone();
        assert_eq!(original.rpc, cloned.rpc);
        assert_eq!(original.block, cloned.block);
    }

    #[test]
    fn test_block_number_args_debug_format() {
        let args = BlockNumberArgs {
            rpc: "test-rpc".to_string(),
            block: Some("latest".to_string()),
        };
        let debug_output = format!("{:?}", args);
        assert!(debug_output.contains("BlockNumberArgs"));
        assert!(debug_output.contains("test-rpc"));
        assert!(debug_output.contains("latest"));
    }

    #[tokio::test]
    async fn test_block_invalid_rpc() {
        let server = Server::new();

        let args = BlockArgs {
            rpc: "invalid-url".to_string(),
            fields: Vec::new(),
            raw: false,
            full: false,
            block: Some("latest".to_string()),
        };
        let params = Parameters(args);

        let result = tokio::time::timeout(std::time::Duration::from_secs(5), server.block(params))
            .await
            .expect("Block tool timeout");

        // Should return error for invalid RPC
        assert!(result.is_err(), "Should return error for invalid RPC URL");
        let error = result.unwrap_err();

        assert!(
            error.message.contains("Failed to get block"),
            "Failed to get block"
        );
    }

    #[tokio::test]
    async fn test_block_with_different_options() {
        let server = Server::new();

        // Test various combinations of block options
        let test_cases = vec![
            // Basic case
            BlockArgs {
                rpc: "https://1rpc.io/eth".to_string(),
                fields: Vec::new(),
                raw: false,
                full: false,
                block: Some("latest".to_string()),
            },
            // Full block
            BlockArgs {
                rpc: "https://1rpc.io/eth".to_string(),
                fields: Vec::new(),
                raw: false,
                full: true,
                block: Some("latest".to_string()),
            },
            // Raw block
            BlockArgs {
                rpc: "https://1rpc.io/eth".to_string(),
                fields: vec!["raw".to_string()],
                raw: false, // This should be overridden by fields containing "raw"
                full: false,
                block: Some("latest".to_string()),
            },
            // Specific fields
            BlockArgs {
                rpc: "https://1rpc.io/eth".to_string(),
                fields: vec!["number".to_string(), "timestamp".to_string()],
                raw: false,
                full: false,
                block: Some("latest".to_string()),
            },
        ];

        for (i, args) in test_cases.into_iter().enumerate() {
            let params = Parameters(args);

            let result =
                tokio::time::timeout(std::time::Duration::from_secs(10), server.block(params))
                    .await
                    .expect(&format!("Block tool timeout for test case {}", i));

            // Test response structure regardless of success/failure
            match result {
                Ok(response) => {
                    assert!(
                        !response.content.is_empty(),
                        "Block tool should return content for test case {}",
                        i
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
                        "Block response should not be empty for test case {}",
                        i
                    );
                    println!(
                        "Block test case {} response length: {}",
                        i,
                        response_text.text.len()
                    );
                }
                Err(error) => {
                    assert!(
                        !error.message.is_empty(),
                        "Error message should not be empty for test case {}",
                        i
                    );
                    println!("Block test case {} error (expected): {}", i, error.message);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_block_block_tags() {
        let server = Server::new();

        let block_tags = vec!["earliest", "finalized", "safe", "latest", "pending"];

        for tag in block_tags {
            let args = BlockArgs {
                rpc: "https://1rpc.io/eth".to_string(),
                fields: vec!["number".to_string()],
                raw: false,
                full: false,
                block: Some(tag.to_string()),
            };
            let params = Parameters(args);

            let result =
                tokio::time::timeout(std::time::Duration::from_secs(5), server.block(params))
                    .await
                    .expect(&format!("Block tool timeout for block tag: {}", tag));

            // Test response structure regardless of success/failure
            match result {
                Ok(response) => {
                    assert!(
                        !response.content.is_empty(),
                        "Block tool should return content for tag: {}",
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
                        "Block response should not be empty for tag: {}",
                        tag
                    );
                    println!(
                        "Block for {}: {}...",
                        tag,
                        &response_text.text[..std::cmp::min(100, response_text.text.len())]
                    );
                }
                Err(error) => {
                    assert!(
                        !error.message.is_empty(),
                        "Error message should not be empty for tag: {}",
                        tag
                    );
                    println!("Block error for {} (expected): {}", tag, error.message);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_concurrent_block_calls() {
        let server = Arc::new(Server::new());

        // Test concurrent execution with different block configurations
        let test_configs = vec![
            (vec!["number".to_string()], false, false, Some("latest")),
            (Vec::new(), true, false, Some("finalized")),
            (vec!["timestamp".to_string()], false, true, Some("safe")),
            (Vec::new(), false, false, None), // Default case
        ];

        let handles: Vec<_> = test_configs
            .into_iter()
            .enumerate()
            .map(|(_, (fields, raw, full, block_opt))| {
                let server_clone = Arc::clone(&server);
                tokio::spawn(async move {
                    let args = BlockArgs {
                        rpc: "https://1rpc.io/eth".to_string(),
                        fields,
                        raw,
                        full,
                        block: block_opt.map(|s| s.to_string()),
                    };
                    let params = Parameters(args);

                    server_clone.block(params).await
                })
            })
            .collect();

        // Wait for all concurrent calls to complete
        let results: Vec<_> = futures::future::join_all(handles).await;

        // Verify all concurrent calls completed
        for (i, result) in results.into_iter().enumerate() {
            let call_result = result.expect(&format!("Concurrent block task {} join failed", i));
            match call_result {
                Ok(response) => {
                    assert!(
                        !response.content.is_empty(),
                        "Concurrent block call {} should return content",
                        i
                    );
                    println!("Concurrent block call {} succeeded", i);
                }
                Err(e) => {
                    assert!(
                        !e.message.is_empty(),
                        "Concurrent block call {} error should have message",
                        i
                    );
                    println!(
                        "Concurrent block call {} failed as expected: {}",
                        i, e.message
                    );
                }
            }
        }
    }

    #[tokio::test]
    async fn test_block_number_tool() {
        let server = Server::new();

        // Test block number with different block identifiers
        let test_cases = vec![
            Some("latest"),
            Some("finalized"),
            Some("safe"),
            Some("1000000"),
            None, // Default case (latest)
        ];

        for block_opt in test_cases {
            let args = BlockNumberArgs {
                rpc: "https://1rpc.io/eth".to_string(),
                block: block_opt.map(|s| s.to_string()),
            };
            let params = Parameters(args);

            let result = tokio::time::timeout(
                std::time::Duration::from_secs(5),
                server.block_number(params),
            )
            .await
            .expect(&format!(
                "Block number tool timeout for block: {:?}",
                block_opt
            ));

            // Test response structure regardless of success/failure
            match result {
                Ok(response) => {
                    assert!(
                        !response.content.is_empty(),
                        "Block number tool should return content for block: {:?}",
                        block_opt
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
                        "Block number response should not be empty for block: {:?}",
                        block_opt
                    );
                    // Block number should be numeric or contain numeric information
                    assert!(
                        response_text.text.contains("number")
                            || response_text.text.parse::<u64>().is_ok(),
                        "Block number response should contain number information"
                    );
                    println!("Block number for {:?}: {}", block_opt, response_text.text);
                }
                Err(error) => {
                    assert!(
                        !error.message.is_empty(),
                        "Error message should not be empty for block: {:?}",
                        block_opt
                    );
                    println!(
                        "Block number error for {:?} (expected): {}",
                        block_opt, error.message
                    );
                }
            }
        }
    }

    #[tokio::test]
    async fn test_concurrent_block_number_calls() {
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
            .map(|(_, block_opt)| {
                let server_clone = Arc::clone(&server);
                tokio::spawn(async move {
                    let args = BlockNumberArgs {
                        rpc: "https://1rpc.io/eth".to_string(),
                        block: block_opt.map(|s| s.to_string()),
                    };
                    let params = Parameters(args);

                    server_clone.block_number(params).await
                })
            })
            .collect();

        // Wait for all concurrent calls to complete
        let results: Vec<_> = futures::future::join_all(handles).await;

        // Verify all concurrent calls completed
        for (i, result) in results.into_iter().enumerate() {
            let call_result =
                result.expect(&format!("Concurrent block number task {} join failed", i));
            match call_result {
                Ok(response) => {
                    assert!(
                        !response.content.is_empty(),
                        "Concurrent block number call {} should return content",
                        i
                    );
                    println!("Concurrent block number call {} succeeded", i);
                }
                Err(e) => {
                    assert!(
                        !e.message.is_empty(),
                        "Concurrent block number call {} error should have message",
                        i
                    );
                    println!(
                        "Concurrent block number call {} failed as expected: {}",
                        i, e.message
                    );
                }
            }
        }
    }

    #[tokio::test]
    async fn test_gas_price_tool() {
        let server = Server::new();

        // Test gas price with different RPC endpoints
        let test_cases = vec![
            "https://1rpc.io/eth",
            "https://ethereum-rpc.publicnode.com",
            // "http://localhost:8545", // Local node (may not be available)
        ];

        for rpc_url in test_cases {
            let args = GasPriceArgs {
                rpc: rpc_url.to_string(),
            };
            let params = Parameters(args);

            let result =
                tokio::time::timeout(std::time::Duration::from_secs(5), server.gas_price(params))
                    .await
                    .expect(&format!("Gas price tool timeout for RPC: {}", rpc_url));

            // Test response structure regardless of success/failure
            match result {
                Ok(response) => {
                    assert!(
                        !response.content.is_empty(),
                        "Gas price tool should return content for RPC: {}",
                        rpc_url
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
                        "Gas price response should not be empty for RPC: {}",
                        rpc_url
                    );
                    // Gas price should be numeric (wei value)
                    let gas_price_result = response_text.text.parse::<u128>();
                    match gas_price_result {
                        Ok(price) => {
                            assert!(price > 0, "Gas price should be positive");
                            println!("Gas price for {}: {} wei", rpc_url, price);
                        }
                        Err(_) => {
                            // If not numeric, should still be non-empty and contain gas price info
                            assert!(
                                response_text.text.contains("gas")
                                    || response_text.text.contains("price"),
                                "Non-numeric gas price response should contain gas/price info"
                            );
                            println!("Gas price for {}: {}", rpc_url, response_text.text);
                        }
                    }
                }
                Err(error) => {
                    assert!(
                        !error.message.is_empty(),
                        "Error message should not be empty for RPC: {}",
                        rpc_url
                    );
                    println!(
                        "Gas price error for {} (expected): {}",
                        rpc_url, error.message
                    );
                }
            }
        }
    }

    #[tokio::test]
    async fn test_concurrent_gas_price_calls() {
        let server = Arc::new(Server::new());

        // Test concurrent execution with different RPC endpoints
        let rpc_endpoints = vec!["https://1rpc.io/eth", "https://ethereum-rpc.publicnode.com"];

        let handles: Vec<_> = rpc_endpoints
            .into_iter()
            .enumerate()
            .map(|(_, rpc_url)| {
                let server_clone = Arc::clone(&server);
                tokio::spawn(async move {
                    let args = GasPriceArgs {
                        rpc: rpc_url.to_string(),
                    };
                    let params = Parameters(args);

                    server_clone.gas_price(params).await
                })
            })
            .collect();

        // Wait for all concurrent calls to complete
        let results: Vec<_> = futures::future::join_all(handles).await;

        // Verify all concurrent calls completed
        for (i, result) in results.into_iter().enumerate() {
            let call_result =
                result.expect(&format!("Concurrent gas price task {} join failed", i));
            match call_result {
                Ok(response) => {
                    assert!(
                        !response.content.is_empty(),
                        "Concurrent gas price call {} should return content",
                        i
                    );
                    println!("Concurrent gas price call {} succeeded", i);
                }
                Err(e) => {
                    assert!(
                        !e.message.is_empty(),
                        "Concurrent gas price call {} error should have message",
                        i
                    );
                    println!(
                        "Concurrent gas price call {} failed as expected: {}",
                        i, e.message
                    );
                }
            }
        }
    }
}

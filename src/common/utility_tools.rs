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

use crate::common::{common::*, server::Server};

#[derive(Debug, serde::Deserialize, DefaultFromSerde, schemars::JsonSchema)]
pub struct MaxIntArgs {
    /// A string representing the integer type. Possible values are int8, int16, int32, int64, int256.
    #[serde(default = "default_int")]
    pub r#type: String,
}

#[derive(Debug, serde::Deserialize, DefaultFromSerde, schemars::JsonSchema)]
pub struct MaxUIntArgs {
    /// A string representing the unsigned integer type. Possible values are uint8, uint16, uint32, uint64, uint256.
    #[serde(default = "default_uint")]
    pub r#type: String,
}

#[tool_router(router = utility_router, vis = "pub")]
impl Server {
    #[tool(description = "A test tool")]
    async fn ping(&self) -> Result<CallToolResult, ErrorData> {
        Ok(CallToolResult::success(vec![Content::text("pong")]))
    }

    #[tool(description = "Get maximum value for integer type.")]
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

    #[tool(description = "Get minimum value for integer type")]
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

    #[tool(description = "Get maximum value for unsigned integer type.")]
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

#[cfg(test)]
mod tests {
    use rmcp::handler::server::wrapper::Parameters;

    use super::*;
    #[test]
    fn test_max_int_args_default() {
        let args = MaxIntArgs::default();
        assert_eq!(args.r#type, "int256");
    }

    #[test]
    fn test_max_uint_args_default() {
        let args = MaxUIntArgs::default();
        assert_eq!(args.r#type, "uint256");
    }

    #[tokio::test]
    async fn test_ping() {
        let server = Server::new();

        // Add timeout control to prevent test hanging
        let result = tokio::time::timeout(std::time::Duration::from_secs(5), server.ping())
            .await
            .expect("Ping request timeout")
            .expect("Ping request failed");

        println!("Result: {:?}", result);

        // Improved error handling and assertions
        let response_text = result
            .content
            .first()
            .ok_or("Response content is empty")
            .and_then(|item| item.raw.as_text().ok_or("Cannot parse as text"))
            .map(|text_item| &text_item.text)
            .expect("Failed to get response text");

        assert_eq!(
            response_text, "pong",
            "Expected 'pong' response, but received: '{}'",
            response_text
        );
    }

    #[tokio::test]
    async fn test_max_int_valid_types() {
        let server = Server::new();

        let valid_types = vec!["int8", "int16", "int32", "int64", "int256"];

        for int_type in valid_types {
            let args = MaxIntArgs {
                r#type: int_type.to_string(),
            };
            let params = Parameters(args);

            let result =
                tokio::time::timeout(std::time::Duration::from_secs(3), server.max_int(params))
                    .await
                    .expect(&format!("Max int {} timeout", int_type))
                    .expect(&format!("Max int {} failed", int_type));

            // Verify response structure
            assert!(
                !result.content.is_empty(),
                "Max int {} should return content",
                int_type
            );

            let response_text = result
                .content
                .first()
                .unwrap()
                .raw
                .as_text()
                .expect(&format!("Max int {} response should be text", int_type));

            assert!(
                !response_text.text.is_empty(),
                "Max int {} should return non-empty text",
                int_type
            );
            println!("Max {}: {}", int_type, response_text.text);
        }
    }

    #[tokio::test]
    async fn test_max_int_invalid_type() {
        let server = Server::new();

        let args = MaxIntArgs {
            r#type: "invalid_type".to_string(),
        };
        let params = Parameters(args);

        let result =
            tokio::time::timeout(std::time::Duration::from_secs(3), server.max_int(params))
                .await
                .expect("Invalid max int timeout");

        // Should return error for invalid type
        assert!(result.is_err(), "Should return error for invalid type");
        let error = result.unwrap_err();
        assert!(
            error.message.contains("Failed to get max int"),
            "Error should mention max int failure"
        );
    }

    #[tokio::test]
    async fn test_min_int_valid_types() {
        let server = Server::new();

        let valid_types = vec!["int8", "int16", "int32", "int64", "int256"];

        for int_type in valid_types {
            let args = MaxIntArgs {
                r#type: int_type.to_string(),
            };
            let params = Parameters(args);

            let result =
                tokio::time::timeout(std::time::Duration::from_secs(3), server.min_int(params))
                    .await
                    .expect(&format!("Min int {} timeout", int_type))
                    .expect(&format!("Min int {} failed", int_type));

            // Verify response structure
            assert!(
                !result.content.is_empty(),
                "Min int {} should return content",
                int_type
            );

            let response_text = result
                .content
                .first()
                .unwrap()
                .raw
                .as_text()
                .expect(&format!("Min int {} response should be text", int_type));

            assert!(
                !response_text.text.is_empty(),
                "Min int {} should return non-empty text",
                int_type
            );
            println!("Min {}: {}", int_type, response_text.text);
        }
    }

    #[tokio::test]
    async fn test_max_uint_valid_types() {
        let server = Server::new();

        let valid_types = vec!["uint8", "uint16", "uint32", "uint64", "uint256"];

        for uint_type in valid_types {
            let args = MaxUIntArgs {
                r#type: uint_type.to_string(),
            };
            let params = Parameters(args);

            let result =
                tokio::time::timeout(std::time::Duration::from_secs(3), server.max_uint(params))
                    .await
                    .expect(&format!("Max uint {} timeout", uint_type))
                    .expect(&format!("Max uint {} failed", uint_type));

            // Verify response structure
            assert!(
                !result.content.is_empty(),
                "Max uint {} should return content",
                uint_type
            );

            let response_text = result
                .content
                .first()
                .unwrap()
                .raw
                .as_text()
                .expect(&format!("Max uint {} response should be text", uint_type));

            assert!(
                !response_text.text.is_empty(),
                "Max uint {} should return non-empty text",
                uint_type
            );
            println!("Max {}: {}", uint_type, response_text.text);
        }
    }

    #[tokio::test]
    async fn test_address_zero() {
        let server = Server::new();

        let result = tokio::time::timeout(std::time::Duration::from_secs(3), server.address_zero())
            .await
            .expect("Address zero timeout")
            .expect("Address zero failed");

        // Verify response structure
        assert!(
            !result.content.is_empty(),
            "Address zero should return content"
        );

        let response_text = result
            .content
            .first()
            .unwrap()
            .raw
            .as_text()
            .expect("Address zero response should be text");

        assert!(
            !response_text.text.is_empty(),
            "Address zero should return non-empty text"
        );
        assert!(
            response_text.text.starts_with("0x"),
            "Address should start with 0x prefix"
        );
        assert_eq!(
            response_text.text.len(),
            42,
            "Ethereum address should be 42 characters"
        );
        println!("Zero address: {}", response_text.text);
    }

    #[tokio::test]
    async fn test_hash_zero() {
        let server = Server::new();

        let result = tokio::time::timeout(std::time::Duration::from_secs(3), server.hash_zero())
            .await
            .expect("Hash zero timeout")
            .expect("Hash zero failed");

        // Verify response structure
        assert!(
            !result.content.is_empty(),
            "Hash zero should return content"
        );

        let response_text = result
            .content
            .first()
            .unwrap()
            .raw
            .as_text()
            .expect("Hash zero response should be text");

        assert!(
            !response_text.text.is_empty(),
            "Hash zero should return non-empty text"
        );
        assert!(
            response_text.text.starts_with("0x"),
            "Hash should start with 0x prefix"
        );
        assert_eq!(
            response_text.text.len(),
            66,
            "Keccak256 hash should be 66 characters"
        );
        println!("Zero hash: {}", response_text.text);
    }

    #[tokio::test]
    async fn test_concurrent_tool_calls() {
        let server = Server::new();

        // Test concurrent execution of different tools
        let handles: Vec<_> = (0..4)
            .map(|i| {
                let server_ref = server.clone();
                tokio::spawn(async move {
                    match i % 4 {
                        0 => server_ref.ping().await,
                        1 => server_ref.address_zero().await,
                        2 => {
                            let args = MaxIntArgs {
                                r#type: "int32".to_string(),
                            };
                            server_ref.max_int(Parameters(args)).await
                        }
                        3 => {
                            let args = MaxUIntArgs {
                                r#type: "uint64".to_string(),
                            };
                            server_ref.max_uint(Parameters(args)).await
                        }
                        _ => unreachable!(),
                    }
                })
            })
            .collect();

        // Wait for all concurrent calls to complete
        let results: Vec<_> = futures::future::join_all(handles).await;

        // Verify all concurrent calls succeeded
        for (i, result) in results.into_iter().enumerate() {
            let call_result = result.expect(&format!("Concurrent task {} join failed", i));
            assert!(call_result.is_ok(), "Concurrent call {} should succeed", i);
        }
    }

    #[tokio::test]
    async fn test_tool_response_consistency() {
        let server = Server::new();

        // Test that multiple calls to the same tool return consistent results
        for _ in 0..3 {
            let result1 = server.ping().await.expect("First ping failed");
            let result2 = server.ping().await.expect("Second ping failed");

            // Compare response content
            assert_eq!(
                result1.content.len(),
                result2.content.len(),
                "Response lengths should match"
            );

            let text1 = result1
                .content
                .first()
                .unwrap()
                .raw
                .as_text()
                .unwrap()
                .text
                .clone();
            let text2 = result2
                .content
                .first()
                .unwrap()
                .raw
                .as_text()
                .unwrap()
                .text
                .clone();

            assert_eq!(text1, text2, "Ping responses should be identical");
        }
    }
}

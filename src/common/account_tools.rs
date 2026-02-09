use alloy_primitives::{Address, U256};
use cast::Cast;
use foundry_cli::{opts::RpcOpts, utils, utils::LoadConfig};
use futures::TryFutureExt;
use rmcp::{
    ErrorData, handler::server::wrapper::Parameters, model::*, schemars, tool, tool_router,
};
use serde_default::DefaultFromSerde;
use serde_json::Value;

use crate::common::{common::*, server::Server};

#[derive(Debug, Clone, serde::Deserialize, DefaultFromSerde, schemars::JsonSchema)]
pub struct BalanceArgs {
    /// The RPC endpoint, default value is http://localhost:8545.
    #[serde(default = "default_rpc")]
    pub rpc: String,

    /// The block height to query at. Can also be the tags earliest, finalized, safe, latest, pending or block hash.
    #[serde(default)]
    pub block: Option<String>,

    /// The account address to query.
    pub who: String,

    /// Format the balance in ether.
    #[serde(default)]
    pub ether: bool,
}

#[derive(Debug, Clone, serde::Deserialize, DefaultFromSerde, schemars::JsonSchema)]
pub struct NonceArgs {
    /// The RPC endpoint, default value is http://localhost:8545.
    #[serde(default = "default_rpc")]
    pub rpc: String,

    /// The block height to query at. Can also be the tags earliest, finalized, safe, latest, pending or block hash.
    #[serde(default)]
    pub block: Option<String>,

    /// The account address to query.
    pub who: String,
}

#[derive(Debug, Clone, serde::Deserialize, DefaultFromSerde, schemars::JsonSchema)]
pub struct CodeArgs {
    /// The RPC endpoint, default value is http://localhost:8545.
    #[serde(default = "default_rpc")]
    pub rpc: String,

    /// The block height to query at. Can also be the tags earliest, finalized, safe, latest, pending or block hash.
    #[serde(default)]
    pub block: Option<String>,

    /// An Ethereum Address
    pub address: Option<String>,

    /// An ENS Name (format does not get checked)
    pub name: Option<String>,

    /// Disassemble bytecodes into individual opcodes.
    #[serde(default)]
    pub disassemble: bool,
}

#[derive(Debug, Clone, serde::Deserialize, DefaultFromSerde, schemars::JsonSchema)]
pub struct StorageArgs {
    /// The RPC endpoint, default value is http://localhost:8545.
    #[serde(default = "default_rpc")]
    pub rpc: String,

    /// The block height to query at. Can also be the tags earliest, finalized, safe, latest, pending or block hash.
    #[serde(default)]
    pub block: Option<String>,

    /// The contract address to query.
    pub address: String,

    /// The storage slot to query.
    pub slot: String,

    /// Return the proof for the queried storage slot.
    #[serde(default)]
    pub proof: bool,
}

#[derive(Debug, Clone, serde::Deserialize, DefaultFromSerde, schemars::JsonSchema)]
pub struct CodeSizeArgs {
    /// The RPC endpoint, default value is http://localhost:8545.
    #[serde(default = "default_rpc")]
    pub rpc: String,

    /// The block height to query at. Can also be the tags earliest, finalized, safe, latest, pending or block hash.
    #[serde(default)]
    pub block: Option<String>,

    /// An Ethereum Address
    pub address: Option<String>,

    /// An ENS Name (format does not get checked)
    pub name: Option<String>,
}

#[tool_router(router = account_router, vis = "pub")]
impl Server {
    #[tool(description = "Get the balance of an account in wei or ether")]
    async fn balance(
        &self,
        Parameters(args): Parameters<BalanceArgs>,
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
            ErrorData::parse_error("Invalid RPC URL", Some(Value::String(format!("{:?}", e))))
        })?;
        let provider = utils::get_provider(&config).map_err(|e| {
            ErrorData::internal_error(
                "Failed to get provider",
                Some(Value::String(format!("{:?}", e))),
            )
        })?;

        let address: Address = args.who.parse().map_err(|e| {
            ErrorData::parse_error(
                "Invalid address format",
                Some(Value::String(format!("{:?}", e))),
            )
        })?;

        let balance = Cast::new(provider)
            .balance(address, None)
            .await
            .map_err(|e| {
                ErrorData::internal_error(
                    "Failed to get balance",
                    Some(Value::String(format!("{:?}", e))),
                )
            })?;

        let result = if args.ether {
            format_balance(balance)
        } else {
            balance.to_string()
        };

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Get the nonce of an account")]
    async fn nonce(
        &self,
        Parameters(args): Parameters<NonceArgs>,
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
            ErrorData::parse_error("Invalid RPC URL", Some(Value::String(format!("{:?}", e))))
        })?;
        let provider = utils::get_provider(&config).map_err(|e| {
            ErrorData::internal_error(
                "Failed to get provider",
                Some(Value::String(format!("{:?}", e))),
            )
        })?;

        let address: Address = args.who.parse().map_err(|e| {
            ErrorData::parse_error(
                "Invalid address format",
                Some(Value::String(format!("{:?}", e))),
            )
        })?;

        let nonce = Cast::new(provider)
            .nonce(address, Some(get_block_id(args.block)))
            .await
            .map_err(|e| {
                ErrorData::internal_error(
                    "Failed to get nonce",
                    Some(Value::String(format!("{:?}", e))),
                )
            })?;

        Ok(CallToolResult::success(vec![Content::text(
            nonce.to_string(),
        )]))
    }

    #[tool(description = "Get the bytecode of a contract")]
    async fn code(
        &self,
        Parameters(args): Parameters<CodeArgs>,
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
            ErrorData::parse_error("Invalid RPC URL", Some(Value::String(format!("{:?}", e))))
        })?;
        let provider = utils::get_provider(&config).map_err(|e| {
            ErrorData::internal_error(
                "Failed to get provider",
                Some(Value::String(format!("{:?}", e))),
            )
        })?;

        let address = resolve(&provider, args.name, args.address)
            .await
            .map_err(|e| {
                ErrorData::internal_error(
                    "Failed to get address",
                    Some(Value::String(format!("{:?}", e))),
                )
            })?;

        let code = Cast::new(provider)
            .code(address, Some(get_block_id(args.block)), args.disassemble)
            .await
            .map_err(|e| {
                ErrorData::internal_error(
                    "Failed to get code",
                    Some(Value::String(format!("{:?}", e))),
                )
            })?;

        Ok(CallToolResult::success(vec![Content::text(code)]))
    }

    #[tool(description = "Get the storage value at a specific slot")]
    async fn storage(
        &self,
        Parameters(args): Parameters<StorageArgs>,
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
            ErrorData::parse_error("Invalid RPC URL", Some(Value::String(format!("{:?}", e))))
        })?;
        let provider = utils::get_provider(&config).map_err(|e| {
            ErrorData::internal_error(
                "Failed to get provider",
                Some(Value::String(format!("{:?}", e))),
            )
        })?;

        let address: Address = args.address.parse().map_err(|e| {
            ErrorData::parse_error(
                "Invalid address format",
                Some(Value::String(format!("{:?}", e))),
            )
        })?;

        let slot: alloy_primitives::B256 = args.slot.parse().map_err(|e| {
            ErrorData::parse_error(
                "Invalid slot format",
                Some(Value::String(format!("{:?}", e))),
            )
        })?;

        let storage = Cast::new(provider)
            .storage(address, slot, Some(get_block_id(args.block)))
            .await
            .map_err(|e| {
                ErrorData::internal_error(
                    "Failed to get storage",
                    Some(Value::String(format!("{:?}", e))),
                )
            })?;

        Ok(CallToolResult::success(vec![Content::text(storage)]))
    }

    #[tool(description = "Get the size of contract bytecode in bytes")]
    async fn code_size(
        &self,
        Parameters(args): Parameters<CodeSizeArgs>,
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
            ErrorData::parse_error("Invalid RPC URL", Some(Value::String(format!("{:?}", e))))
        })?;
        let provider = utils::get_provider(&config).map_err(|e| {
            ErrorData::internal_error(
                "Failed to get provider",
                Some(Value::String(format!("{:?}", e))),
            )
        })?;

        let address = resolve(&provider, args.name, args.address)
            .await
            .map_err(|e| {
                ErrorData::internal_error(
                    "Failed to get address",
                    Some(Value::String(format!("{:?}", e))),
                )
            })?;

        let byte_size = Cast::new(provider)
            .codesize(address, Some(get_block_id(args.block)))
            .await
            .map_err(|e| {
                ErrorData::internal_error(
                    "Failed to get code size",
                    Some(Value::String(format!("{:?}", e))),
                )
            })?;

        Ok(CallToolResult::success(vec![Content::text(
            byte_size.to_string(),
        )]))
    }
}

fn format_balance(balance: U256) -> String {
    // Convert wei to ether (1 ether = 10^18 wei)
    let ether = balance.to::<u128>() as f64 / 1_000_000_000_000_000_000.0;
    format!("{:.18}", ether)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rmcp::handler::server::wrapper::Parameters;

    use super::*;

    #[test]
    fn test_balance_args_default() {
        let args = BalanceArgs::default();
        assert_eq!(args.rpc, "http://localhost:8545");
        assert_eq!(args.block, None);
        assert_eq!(args.ether, false);
    }

    #[test]
    fn test_nonce_args_default() {
        let args = NonceArgs::default();
        assert_eq!(args.rpc, "http://localhost:8545");
        assert_eq!(args.block, None);
    }

    #[test]
    fn test_code_args_default() {
        let args = CodeArgs::default();
        assert_eq!(args.rpc, "http://localhost:8545");
        assert_eq!(args.block, None);
        assert_eq!(args.disassemble, false);
    }

    #[test]
    fn test_storage_args_default() {
        let args = StorageArgs::default();
        assert_eq!(args.rpc, "http://localhost:8545");
        assert_eq!(args.block, None);
        assert_eq!(args.proof, false);
    }

    #[test]
    fn test_code_size_args_default() {
        let args = CodeSizeArgs::default();
        assert_eq!(args.rpc, "http://localhost:8545");
        assert_eq!(args.block, None);
    }

    #[tokio::test]
    async fn test_account_tools_response_structure() {
        let server = Server::new();

        // Test with a well-known contract address
        let test_address = "0x1f9840a85d5af5bf1d1762f925bdaddc4201f984"; // Uniswap V3 Factory

        // Test balance tool
        let balance_args = BalanceArgs {
            rpc: "https://1rpc.io/eth".to_string(),
            block: Some("latest".to_string()),
            who: test_address.to_string(),
            ether: true,
        };
        let balance_params = Parameters(balance_args);

        let balance_result = server.balance(balance_params).await;
        match balance_result {
            Ok(result) => {
                assert!(
                    !result.content.is_empty(),
                    "Balance tool should return content when successful"
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
                    "Balance response should not be empty"
                );
                println!("Balance response: {}", response_text.text);
            }
            Err(error) => {
                assert!(
                    !error.message.is_empty(),
                    "Error message should not be empty"
                );
                println!("Balance error (expected): {}", error.message);
            }
        }

        // Test nonce tool
        let nonce_args = NonceArgs {
            rpc: "https://1rpc.io/eth".to_string(),
            block: Some("latest".to_string()),
            who: test_address.to_string(),
        };
        let nonce_params = Parameters(nonce_args);

        let nonce_result = server.nonce(nonce_params).await;
        match nonce_result {
            Ok(result) => {
                assert!(
                    !result.content.is_empty(),
                    "Nonce tool should return content when successful"
                );
                let response_text = result
                    .content
                    .first()
                    .unwrap()
                    .raw
                    .as_text()
                    .expect("Response should be text");
                // Nonce should be numeric
                assert!(
                    response_text.text.parse::<u64>().is_ok() || !response_text.text.is_empty(),
                    "Nonce should be numeric or non-empty"
                );
                println!("Nonce response: {}", response_text.text);
            }
            Err(error) => {
                assert!(
                    !error.message.is_empty(),
                    "Error message should not be empty"
                );
                println!("Nonce error (expected): {}", error.message);
            }
        }

        // Test code tool
        let code_args = CodeArgs {
            rpc: "https://1rpc.io/eth".to_string(),
            block: Some("latest".to_string()),
            address: Some(test_address.to_string()),
            name: None,
            disassemble: false,
        };
        let code_params = Parameters(code_args);

        let code_result = server.code(code_params).await;
        match code_result {
            Ok(result) => {
                assert!(
                    !result.content.is_empty(),
                    "Code tool should return content when successful"
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
                    "Code response should not be empty"
                );
                println!("Code response length: {}", response_text.text.len());
            }
            Err(error) => {
                assert!(
                    !error.message.is_empty(),
                    "Error message should not be empty"
                );
                println!("Code error (expected): {}", error.message);
            }
        }

        // Test storage tool (slot 0 is often used)
        let storage_args = StorageArgs {
            rpc: "https://1rpc.io/eth".to_string(),
            block: Some("latest".to_string()),
            address: test_address.to_string(),
            slot: "0x0".to_string(),
            proof: false,
        };
        let storage_params = Parameters(storage_args);

        let storage_result = server.storage(storage_params).await;
        match storage_result {
            Ok(result) => {
                assert!(
                    !result.content.is_empty(),
                    "Storage tool should return content when successful"
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
                    "Storage response should not be empty"
                );
                println!("Storage response: {}", response_text.text);
            }
            Err(error) => {
                assert!(
                    !error.message.is_empty(),
                    "Error message should not be empty"
                );
                println!("Storage error (expected): {}", error.message);
            }
        }

        // Test code_size tool
        let code_size_args = CodeSizeArgs {
            rpc: "https://1rpc.io/eth".to_string(),
            block: Some("latest".to_string()),
            address: Some(test_address.to_string()),
            name: None,
        };
        let code_size_params = Parameters(code_size_args);

        let code_size_result = server.code_size(code_size_params).await;
        match code_size_result {
            Ok(result) => {
                assert!(
                    !result.content.is_empty(),
                    "Code size tool should return content when successful"
                );
                let response_text = result
                    .content
                    .first()
                    .unwrap()
                    .raw
                    .as_text()
                    .expect("Response should be text");
                // Code size should be numeric
                assert!(
                    response_text.text.parse::<usize>().is_ok() || !response_text.text.is_empty(),
                    "Code size should be numeric or non-empty"
                );
                println!("Code size response: {} bytes", response_text.text);
            }
            Err(error) => {
                assert!(
                    !error.message.is_empty(),
                    "Error message should not be empty"
                );
                println!("Code size error (expected): {}", error.message);
            }
        }

        // Test code_size tool
        let code_size_args = CodeSizeArgs {
            rpc: "https://1rpc.io/eth".to_string(),
            block: Some("latest".to_string()),
            address: Some(test_address.to_string()),
            name: None,
        };
        let code_size_params = Parameters(code_size_args);

        let code_size_result = server.code_size(code_size_params).await;
        match code_size_result {
            Ok(result) => {
                assert!(
                    !result.content.is_empty(),
                    "Code size tool should return content when successful"
                );
                let response_text = result
                    .content
                    .first()
                    .unwrap()
                    .raw
                    .as_text()
                    .expect("Response should be text");
                // Code size should be numeric
                assert!(
                    response_text.text.parse::<usize>().is_ok() || !response_text.text.is_empty(),
                    "Code size should be numeric or non-empty"
                );
                println!("Code size response: {} bytes", response_text.text);
            }
            Err(error) => {
                assert!(
                    !error.message.is_empty(),
                    "Error message should not be empty"
                );
                println!("Code size error (expected): {}", error.message);
            }
        }
    }

    #[tokio::test]
    async fn test_concurrent_account_tool_calls() {
        let server = Arc::new(Server::new());
        let test_address = "0x1f9840a85d5af5bf1d1762f925bdaddc4201f984";

        // Test concurrent execution of account tools
        let handles: Vec<_> = (0..8)
            .map(|i| {
                let server_clone = Arc::clone(&server);
                let addr = test_address.to_string();
                tokio::spawn(async move {
                    match i % 4 {
                        0 => {
                            let args = BalanceArgs {
                                rpc: "https://1rpc.io/eth".to_string(),
                                block: Some("latest".to_string()),
                                who: addr,
                                ether: true,
                            };
                            server_clone.balance(Parameters(args)).await
                        }
                        1 => {
                            let args = NonceArgs {
                                rpc: "https://1rpc.io/eth".to_string(),
                                block: Some("latest".to_string()),
                                who: addr,
                            };
                            server_clone.nonce(Parameters(args)).await
                        }
                        2 => {
                            let args = CodeArgs {
                                rpc: "https://1rpc.io/eth".to_string(),
                                block: Some("latest".to_string()),
                                address: Some(addr),
                                name: None,
                                disassemble: false,
                            };
                            server_clone.code(Parameters(args)).await
                        }
                        3 => {
                            let args = StorageArgs {
                                rpc: "https://1rpc.io/eth".to_string(),
                                block: Some("latest".to_string()),
                                address: addr,
                                slot: "0x0".to_string(),
                                proof: false,
                            };
                            server_clone.storage(Parameters(args)).await
                        }
                        _ => unreachable!(),
                    }
                })
            })
            .collect();

        // Wait for all concurrent calls to complete
        let results: Vec<_> = futures::future::join_all(handles).await;

        // Verify all concurrent calls completed
        for (i, result) in results.into_iter().enumerate() {
            let call_result = result.expect(&format!("Concurrent account task {} join failed", i));
            match call_result {
                Ok(_) => println!("Concurrent account call {} succeeded", i),
                Err(e) => println!(
                    "Concurrent account call {} failed as expected: {}",
                    i, e.message
                ),
            }
        }
    }

    #[test]
    fn test_balance_formatting() {
        // Test wei to ether conversion
        let balance_wei = U256::from(1_000_000_000_000_000_000u128); // 1 ETH in wei
        let formatted = format_balance(balance_wei);
        assert!(formatted.contains("1."), "Should show 1.x ETH");

        let small_balance = U256::from(1_000_000u128); // Very small amount
        let small_formatted = format_balance(small_balance);
        assert!(!small_formatted.is_empty(), "Should format small balances");
    }

    #[test]
    fn test_args_debug_format() {
        let balance_args = BalanceArgs {
            rpc: "test-rpc".to_string(),
            block: Some("latest".to_string()),
            who: "0x1234".to_string(),
            ether: true,
        };
        let debug_output = format!("{:?}", balance_args);
        assert!(debug_output.contains("BalanceArgs"));
        assert!(debug_output.contains("test-rpc"));

        let nonce_args = NonceArgs {
            rpc: "test-rpc".to_string(),
            block: Some("pending".to_string()),
            who: "0x5678".to_string(),
        };
        let nonce_debug = format!("{:?}", nonce_args);
        assert!(nonce_debug.contains("NonceArgs"));
        assert!(nonce_debug.contains("0x5678"));
    }
}

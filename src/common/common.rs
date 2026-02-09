use std::io;

use alloy_ens::ProviderEnsExt;
use alloy_primitives::{Address, B256, Keccak256, address, hex};
use alloy_rpc_types::{BlockId, BlockNumberOrTag::Latest};

//use alloy_ens::contract::EnsError;
pub fn default_rpc() -> String {
    "http://localhost:8545".to_string()
}
pub fn default_int() -> String {
    "int256".to_string()
}

pub fn default_uint() -> String {
    "uint256".to_string()
}

pub fn get_block_id(block: Option<String>) -> BlockId {
    match block {
        Some(block_str) => {
            let trimmed = block_str.trim();

            match trimmed.to_lowercase().as_str() {
                "latest" => return BlockId::Number(Latest),
                "earliest" => return BlockId::Number(alloy_rpc_types::BlockNumberOrTag::Earliest),
                "pending" => return BlockId::Number(alloy_rpc_types::BlockNumberOrTag::Pending),
                "safe" => return BlockId::Number(alloy_rpc_types::BlockNumberOrTag::Safe),
                "finalized" => {
                    return BlockId::Number(alloy_rpc_types::BlockNumberOrTag::Finalized);
                }
                _ => {}
            }

            if trimmed.starts_with("0x") && trimmed.len() == 66 {
                if let Ok(hash_bytes) = hex::decode(&trimmed[2..]) {
                    if let Ok(hash) = alloy_primitives::B256::try_from(hash_bytes.as_slice()) {
                        return BlockId::Hash(alloy_rpc_types::RpcBlockHash::from_hash(hash, None));
                    }
                }
            }

            if let Ok(num) = trimmed.parse::<u64>() {
                return BlockId::Number(num.into());
            }

            BlockId::Number(Latest)
        }
        None => BlockId::Number(Latest),
    }
}

pub async fn resolve<N: alloy_provider::Network, P: alloy_provider::Provider<N>>(
    provider: &P,
    name: Option<String>,
    address: Option<String>,
) -> Result<Address, String> {
    if let Some(name) = name {
        provider
            .resolve_name(&name)
            .await
            .map_err(|e| e.to_string())?;
    }
    if let Some(address) = address {
        Address::from_slice(hex::decode(address).unwrap().as_slice());
    }
    Err("address is empty".to_string())
}

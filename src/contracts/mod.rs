//! Contains all related smart contract addresses. Modified from [`addressbook`].
//!
//! Sources:
//! - UniswapV2
//!   - Factory: https://docs.uniswap.org/protocol/V2/reference/smart-contracts/factory
//!   - Router: https://docs.uniswap.org/protocol/V2/reference/smart-contracts/router-02
//! - UniswapV3: https://docs.uniswap.org/protocol/reference/deployments
//! - Sushiswap: https://docs.sushi.com/docs/Developers/Deployment%20Addresses
//! - Quickswap
//!   - Factory: https://docs.quickswap.exchange/reference/smart-contracts/01-factory
//!   - Router: https://docs.quickswap.exchange/reference/smart-contracts/router02
//! - Spookyswap: https://docs.spooky.fi/Resources/contracts
//! - Traderjoe: https://docs.traderjoexyz.com/en/security-and-contracts/contracts
//!
//! [`addressbook`]: ethers::addressbook

use ethers::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

const CONTRACTS_JSON: &str = include_str!("./contracts.json");

static CONTRACTS_ADDRESS_BOOK: Lazy<HashMap<String, Contract>> =
    Lazy::new(|| serde_json::from_str(CONTRACTS_JSON).unwrap());

/// Wrapper around a hash map that maps a [Chain] to the contract's deployed address on that chain.
#[derive(Clone, Debug, Deserialize)]
pub struct Contract {
    addresses: HashMap<Chain, Address>,
}

impl Contract {
    /// Returns the address of the contract on the specified chain. Returns None if the contract's
    /// address is not found in the addressbook.
    pub fn address(&self, chain: Chain) -> Option<Address> {
        self.addresses.get(&chain).cloned()
    }
}

/// Fetch the addressbook for a contract by its name. If the contract name is not a part of the
/// address book we return None.
pub fn try_contract<S: Into<String>>(name: S) -> Option<Contract> {
    CONTRACTS_ADDRESS_BOOK.get(&name.into()).cloned()
}

/// Fetch the address for a contract by its name and chain. If the contract name is not a part of
/// the address book we return None.
pub fn try_address<S: Into<String>>(name: S, chain: Chain) -> Option<Address> {
    let c = try_contract(&name.into());
    c.and_then(|c| c.address(chain))
}

/// Fetch the addressbook for a contract by its name. If the contract name is not a part of the
/// address book we panic.
pub fn contract<S: Into<String>>(name: S) -> Contract {
    let name: String = name.into();
    try_contract(&name).unwrap_or_else(|| {
        panic!("uniswap_rs::contracts: \"{}\" is not present in addressbook", name)
    })
}

/// Fetch the address for a contract by its name and chain. If the contract name is not a part of
/// the address book we panic.
pub fn address<S: Into<String>>(name: S, chain: Chain) -> Address {
    let name: String = name.into();
    let contract = contract(&name);
    contract.address(chain).unwrap_or_else(|| {
        panic!("uniswap_rs::contracts: Chain \"{:?}\" for contract \"{}\" is not present in addressbook", chain, name)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contracts() {
        assert!(try_contract("DAI").is_some());
        assert!(try_contract("USDC").is_some());
        assert!(try_contract("rand").is_none());
    }

    #[test]
    fn test_addresses() {
        assert!(try_contract("DAI").unwrap().address(Chain::Mainnet).is_some());
        assert!(try_contract("DAI").unwrap().address(Chain::MoonbeamDev).is_none());

        assert!(try_address("DAI", Chain::Mainnet).is_some());
        assert!(try_address("DAI", Chain::MoonbeamDev).is_none());
    }
}

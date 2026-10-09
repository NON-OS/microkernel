//! The public `0x` account: its address, its balances on each network, and
//! where a sent transaction stands. Sends are in `account_send`.

use super::Wallet;
use crate::error::WalletError;
use crate::evm::{self, checksummed, Network, SendStatus};
use crate::ffi::account_types::PublicBalances;
use crate::ffi::evm::parse_evm_address;

impl Wallet {
    /// The public account's address, with its checksum casing.
    pub fn public_address(&self) -> Result<String, WalletError> {
        self.with(|s| checksummed(&s.evm().address()))
    }

    /// The account's ETH and NOX on `network`, read now over Tor.
    pub fn public_balances(&self, network: Network) -> Result<PublicBalances, WalletError> {
        let of = self.with(|s| s.evm().address())?;
        let held = evm::balances(self.tor()?.as_ref(), network, &of)?;
        Ok(PublicBalances {
            network,
            address: checksummed(&of),
            eth: evm::units::ether(held.eth),
            nox: evm::units::ether(held.nox),
            usdc: evm::units::format(held.usdc, 6),
        })
    }

    /// The most of `coin` spendable on `network`: all of a token, ether less a swap fee.
    pub fn public_max(
        &self,
        network: Network,
        coin: crate::net::asset::Coin,
    ) -> Result<String, WalletError> {
        let of = self.with(|s| s.evm().address())?;
        let max = evm::max_spend(self.tor()?.as_ref(), network, &of, coin)?;
        Ok(evm::units::format(max, coin.decimals()))
    }

    /// Where a sent transaction stands.
    pub fn public_send_status(
        &self,
        network: Network,
        hash: String,
    ) -> Result<SendStatus, WalletError> {
        let bytes = crate::net::rpc::hex_bytes(hash.trim()).ok_or(WalletError::Address)?;
        let hash = <[u8; 32]>::try_from(bytes.as_slice()).map_err(|_| WalletError::Address)?;
        Ok(evm::status(self.tor()?.as_ref(), network, &hash)?)
    }
}

/// A recipient, with its checksum held to when it is written in mixed case:
/// a mixed-case address with a wrong checksum is a typo, not an address.
pub(super) fn parse_recipient(text: &str) -> Result<[u8; 20], WalletError> {
    let address = parse_evm_address(text)?;
    let digits = text.trim().trim_start_matches("0x");
    let mixed = digits.chars().any(|c| c.is_ascii_uppercase())
        && digits.chars().any(|c| c.is_ascii_lowercase());
    if mixed && checksummed(&address) != format!("0x{digits}") {
        return Err(WalletError::Address);
    }
    Ok(address)
}

/// Whether the shielded pool exists on `network`: Sepolia, while it is the only one deployed.
pub fn network_has_shield(network: Network) -> bool {
    network.chain().shield
}

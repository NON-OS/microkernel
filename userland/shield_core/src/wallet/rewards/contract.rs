//! A mainnet address that is a contract wallet signs by ERC-1271, which only mainnet can check, so
//! it is asked there over Tor, as the reward script asks it at the first block of an epoch.

use crate::error::NetError;
use crate::evm::{view, Network, Viewed};
use crate::net::tor::Tor;
use crate::wallet::abi::{bytes, word};

/// `isValidSignature(bytes32,bytes)`, which is also the value a wallet returns to accept.
const IS_VALID_SIGNATURE: [u8; 4] = [0x16, 0x26, 0xba, 0x7e];

/// Whether the contract at `mainnet` accepts `signature` over `digest` on mainnet now.
pub fn accepts(
    tor: &Tor,
    mainnet: &[u8; 20],
    digest: &[u8; 32],
    signature: &[u8],
) -> Result<bool, NetError> {
    let mut ask = IS_VALID_SIGNATURE.to_vec();
    ask.extend_from_slice(digest);
    ask.extend_from_slice(&word(2 * 32));
    ask.extend_from_slice(&bytes(signature));
    let found = view(tor, Network::Mainnet, None, mainnet, &[ask])?;
    Ok(match found.first() {
        Some(Viewed::Data(reply)) => reply.get(..4) == Some(IS_VALID_SIGNATURE.as_slice()),
        _ => false,
    })
}

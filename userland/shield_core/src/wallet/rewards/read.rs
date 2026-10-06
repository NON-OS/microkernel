//! What the registry holds for a testnet address and the mainnet address it would link, read over
//! Tor on the circuits of the Sepolia account: the mainnet address the testnet one is linked to,
//! the link of the mainnet address, its next nonce, and the epoch now.

use super::calls::{of, CURRENT_EPOCH, LINK_OF, MAINNET_OF, NONCES, REGISTRY_BYTES};
use crate::error::NetError;
use crate::evm::{view, Network, Viewed};
use crate::net::tor::Tor;

/// The registry for one pair of addresses.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LinkState {
    /// The mainnet address the testnet address is linked to, then the link of the mainnet address.
    pub linked_to: Option<[u8; 20]>,
    pub testnet: Option<[u8; 20]>,
    pub from_epoch: u64,
    pub changed_in: u64,
    pub contract_wallet: bool,
    pub nonce: u128,
    pub started: bool,
    pub epoch: u64,
}

pub fn read_link(tor: &Tor, testnet: &[u8; 20], mainnet: &[u8; 20]) -> Result<LinkState, NetError> {
    let asks = [of(MAINNET_OF, testnet), of(LINK_OF, mainnet), of(NONCES, mainnet)];
    let asks = [asks.as_slice(), &[CURRENT_EPOCH.to_vec()]].concat();
    let found = view(tor, Network::Sepolia, None, &REGISTRY_BYTES, &asks)?;
    link_state(&found).ok_or(NetError::ReplyShape)
}

/// The four replies read strictly: a word that is not the shape its field takes refuses them all.
pub(crate) fn link_state(found: &[Viewed]) -> Option<LinkState> {
    let data = |i: usize| match found.get(i) {
        Some(Viewed::Data(d)) => Some(d.as_slice()),
        _ => None,
    };
    let (link, epoch) = (data(1)?, data(3)?);
    Some(LinkState {
        linked_to: address(at(data(0)?, 0)?)?,
        testnet: address(at(link, 0)?)?,
        from_epoch: number(at(link, 1))?,
        changed_in: number(at(link, 2))?,
        contract_wallet: number(at(link, 3)).filter(|b| *b < 2)? == 1,
        nonce: wide(at(data(2)?, 0))?,
        started: number(at(epoch, 0)).filter(|b| *b < 2)? == 1,
        epoch: number(at(epoch, 1))?,
    })
}

fn at(data: &[u8], i: usize) -> Option<&[u8; 32]> {
    data.get(i.checked_mul(32)?..i.checked_add(1)?.checked_mul(32)?)?.try_into().ok()
}

/// An address word, none for zero, and a refusal for a word with high bytes set.
fn address(word: &[u8; 32]) -> Option<Option<[u8; 20]>> {
    let (high, low) = word.split_at(12);
    if high.iter().any(|b| *b != 0) {
        return None;
    }
    Some(low.iter().any(|b| *b != 0).then(|| low.try_into().ok()).flatten())
}

fn wide(word: Option<&[u8; 32]>) -> Option<u128> {
    let (high, low) = word?.split_at(16);
    high.iter().all(|b| *b == 0).then(|| low.try_into().ok().map(u128::from_be_bytes))?
}

fn number(word: Option<&[u8; 32]>) -> Option<u64> {
    wide(word).and_then(|n| u64::try_from(n).ok())
}

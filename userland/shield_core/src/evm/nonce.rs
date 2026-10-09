//! The sends this device made, per network and address, so a new send never
//! reuses a waiting nonce. A read RPC cannot see a private relay's pool, so each
//! review asks the relays which recorded sends they hold and signs one past the
//! highest. A dropped send is not held, so the account never sticks. The file
//! holds only public nonces and hashes. `spec/Account.tla` checks it.

use super::network::Network;
use std::path::{Path, PathBuf};

/// How many recent sends are kept.
const KEPT: usize = 16;

fn file(dir: &Path, network: Network, of: &[u8; 20]) -> PathBuf {
    let name: String = of.iter().map(|b| format!("{b:02x}")).collect();
    dir.join(network.slug()).join(name)
}

/// This device's recent sends on `network`: each nonce with its hash.
pub(crate) fn recorded(dir: &Path, network: Network, of: &[u8; 20]) -> Vec<(u64, [u8; 32])> {
    let text = std::fs::read_to_string(file(dir, network, of)).unwrap_or_default();
    text.lines().filter_map(line).collect()
}

pub(super) fn line(text: &str) -> Option<(u64, [u8; 32])> {
    let (nonce, hash) = text.split_once(' ')?;
    let bytes = crate::net::rpc::hex_bytes(hash)?;
    Some((nonce.parse().ok()?, bytes.as_slice().try_into().ok()?))
}

/// Record a send, written to a side file and renamed over the old one.
pub(crate) fn record(
    dir: &Path,
    network: Network,
    of: &[u8; 20],
    nonce: u64,
    hash: &[u8; 32],
) -> std::io::Result<()> {
    let mut sends = recorded(dir, network, of);
    sends.push((nonce, *hash));
    sends.sort_by_key(|(n, _)| *n);
    let keep = sends.len().saturating_sub(KEPT);
    let text: String = sends
        .get(keep..)
        .unwrap_or_default()
        .iter()
        .map(|(n, h)| {
            format!("{n} 0x{}\n", h.iter().map(|b| format!("{b:02x}")).collect::<String>())
        })
        .collect();
    let path = file(dir, network, of);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let side = path.with_extension("new");
    std::fs::write(&side, text)?;
    std::fs::rename(side, path)
}

#[cfg(test)]
#[path = "nonce_test.rs"]
mod nonce_test;

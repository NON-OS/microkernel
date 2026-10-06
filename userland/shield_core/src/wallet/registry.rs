//! Which of the pool's newest roots the association registry holds. A spend is settled only
//! against a registered association root, and the wallet uses its note root as that root, so the
//! anchor is the newest committed root the registry holds. One batch over the scan circuits.

use crate::error::NetError;
use crate::net::pool::ACTIVE;
use crate::net::rpc::{calls_at_over_tor, RawLog};
use crate::net::tor::{Purpose, Tor};

/// `isRegisteredRoot(bytes32)`.
const IS_REGISTERED: [u8; 4] = [0xfe, 0x54, 0xd0, 0xde];
/// The newest roots asked about. Older ones have left the pool's root window.
const ASKED: usize = 8;

/// The registered roots among the newest committed ones, or none when the pool has no registry.
pub fn registered(
    tor: &Tor,
    host: &str,
    roots: &[RawLog],
    head: u64,
) -> Result<Option<Vec<[u8; 32]>>, NetError> {
    let Some(registry) = ACTIVE.registry else { return Ok(None) };
    let newest: Vec<[u8; 32]> =
        roots.iter().rev().filter_map(|r| r.topics.get(1).copied()).take(ASKED).collect();
    let datas: Vec<Vec<u8>> =
        newest.iter().map(|root| [IS_REGISTERED.as_slice(), root].concat()).collect();
    let answers = calls_at_over_tor(tor, Purpose::Scan, host, registry, &datas, head)?;
    Ok(Some(
        newest
            .into_iter()
            .zip(answers)
            .filter(|(_, a)| a.last() == Some(&1))
            .map(|(root, _)| root)
            .collect(),
    ))
}

/// The test an anchor applies: any root when the pool has no registry, else a registered one.
pub fn allows(registered: &Option<Vec<[u8; 32]>>) -> impl Fn(&[u8; 32]) -> bool + '_ {
    move |root| registered.as_ref().is_none_or(|list| list.contains(root))
}

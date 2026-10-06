//! The calls a NOX transfer depends on: its calldata, and the reads of the
//! token's state and a simulation of the transfer as the sender.

use super::calls::{call, storage};
use super::network::Chain;
use super::rpc::Call;

/// ERC-1967's implementation slot.
const IMPLEMENTATION: [u8; 32] = [
    0x36, 0x08, 0x94, 0xa1, 0x3b, 0xa1, 0xa3, 0x21, 0x06, 0x67, 0xc8, 0x28, 0x49, 0x2d, 0xb9, 0x8d,
    0xca, 0x3e, 0x20, 0x76, 0xcc, 0x37, 0x35, 0xa9, 0x20, 0xa3, 0xca, 0x50, 0x5d, 0x38, 0x2b, 0xbc,
];
pub(super) const TRANSFER: [u8; 4] = [0xa9, 0x05, 0x9c, 0xbb];
pub(super) const BALANCE_OF: [u8; 4] = [0x70, 0xa0, 0x82, 0x31];
const PAUSED: [u8; 4] = [0x5c, 0x97, 0x5a, 0xbb];
const BLACKLISTED: [u8; 4] = [0xdb, 0xac, 0x26, 0xe9];
const FEES: [u8; 4] = [0x9a, 0xf1, 0xd3, 0x5a];
const IS_PAIR: [u8; 4] = [0xe5, 0xe3, 0x1b, 0x13];
const FEE_EXEMPT: [u8; 4] = [0x39, 0x8d, 0xaa, 0x85];

/// How many answers `reads` asks for, in the order `rules` reads them.
pub(super) const READS: usize = 10;

/// A selector and one address argument.
pub(super) fn with_address(selector: [u8; 4], who: &[u8; 20]) -> Vec<u8> {
    let mut out = selector.to_vec();
    out.extend_from_slice(&[0u8; 12]);
    out.extend_from_slice(who);
    out
}

/// `transfer(to, amount)`.
pub(super) fn transfer(to: &[u8; 20], amount: u128) -> Vec<u8> {
    let mut out = with_address(TRANSFER, to);
    out.extend_from_slice(&[0u8; 16]);
    out.extend_from_slice(&amount.to_be_bytes());
    out
}

pub(super) fn reads(chain: &Chain, from: &[u8; 20], to: &[u8; 20], amount: u128) -> Vec<Call> {
    let t = &chain.nox_token;
    vec![
        storage(t, &IMPLEMENTATION),
        call(None, t, &PAUSED),
        call(None, t, &with_address(BLACKLISTED, from)),
        call(None, t, &with_address(BLACKLISTED, to)),
        call(None, t, &FEES),
        call(None, t, &with_address(IS_PAIR, from)),
        call(None, t, &with_address(IS_PAIR, to)),
        call(None, t, &with_address(FEE_EXEMPT, from)),
        call(None, t, &with_address(FEE_EXEMPT, to)),
        call(Some(from), t, &transfer(to, amount)),
    ]
}

//! The reads a swap quote rests on, in one batch: the account, the fees, each
//! pair's reserves and token order, the input token's balance and allowance,
//! and when NOX is on the route, every piece of its state a transfer depends
//! on.

use super::route::Hop;
use super::venue::Venue;
use crate::evm::calls::{balance, call, latest_block, nonce, priority_fee, storage};
use crate::evm::network::Chain;
use crate::evm::nox::{with_address, BALANCE_OF};
use crate::evm::rpc::Call;

pub(super) const GET_RESERVES: [u8; 4] = [0x09, 0x02, 0xf1, 0xac];
pub(super) const TOKEN0: [u8; 4] = [0x0d, 0xfe, 0x16, 0x81];
const ALLOWANCE: [u8; 4] = [0xdd, 0x62, 0xed, 0x3e];
pub(super) const IMPLEMENTATION: [u8; 32] = [
    0x36, 0x08, 0x94, 0xa1, 0x3b, 0xa1, 0xa3, 0x21, 0x06, 0x67, 0xc8, 0x28, 0x49, 0x2d, 0xb9, 0x8d,
    0xca, 0x3e, 0x20, 0x76, 0xcc, 0x37, 0x35, 0xa9, 0x20, 0xa3, 0xca, 0x50, 0x5d, 0x38, 0x2b, 0xbc,
];
/// paused, blacklisted(from), fees, feeExempt(from), autoSwapEnabled,
/// autoSwapThreshold, maxAutoSwapChunk, v2Initialized.
const NOX_VIEWS: [[u8; 4]; 8] = [
    [0x5c, 0x97, 0x5a, 0xbb],
    [0xdb, 0xac, 0x26, 0xe9],
    [0x9a, 0xf1, 0xd3, 0x5a],
    [0x39, 0x8d, 0xaa, 0x85],
    [0xbe, 0x80, 0xb0, 0x5b],
    [0xf4, 0x4a, 0x5f, 0xb9],
    [0xf5, 0x54, 0x34, 0x93],
    [0x5d, 0x0d, 0xc3, 0x4e],
];
/// How many answers the NOX part holds: the slot, the eight views, and the
/// fees the token itself holds.
pub(super) const NOX_READS: usize = 10;

pub(super) fn calls(
    chain: &Chain,
    venue: &Venue,
    from: &[u8; 20],
    hops: &[Hop],
    nox: bool,
) -> Vec<Call> {
    let mut out = vec![balance(from), nonce(from), latest_block(), priority_fee()];
    for hop in hops {
        out.push(call(None, &hop.pair.address, &GET_RESERVES));
        out.push(call(None, &hop.pair.address, &TOKEN0));
    }
    if let Some(first) = hops.first().filter(|h| h.token_in != venue.weth) {
        let mut allowance = with_address(ALLOWANCE, from);
        allowance.extend_from_slice(&[0u8; 12]);
        allowance.extend_from_slice(&venue.router);
        out.push(call(None, &first.token_in, &with_address(BALANCE_OF, from)));
        out.push(call(None, &first.token_in, &allowance));
    }
    if nox {
        let t = &chain.nox_token;
        out.push(storage(t, &IMPLEMENTATION));
        for (i, selector) in NOX_VIEWS.iter().enumerate() {
            let data =
                if i == 1 || i == 3 { with_address(*selector, from) } else { selector.to_vec() };
            out.push(call(None, t, &data));
        }
        out.push(call(None, t, &with_address(BALANCE_OF, t)));
    }
    out
}

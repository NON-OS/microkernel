//! The router calls a swap makes, and the approval before one. Only the
//! fee-on-transfer variants are used: they check what actually reaches the
//! recipient against the minimum, which is the only honest check for NOX,
//! whose transfers take a fee, and costs nothing extra for USDC and WETH.

use crate::wallet::abi::word;

const ETH_FOR_TOKENS: [u8; 4] = [0xb6, 0xf9, 0xde, 0x95];
const TOKENS_FOR_ETH: [u8; 4] = [0x79, 0x1a, 0xc9, 0x47];
const TOKENS_FOR_TOKENS: [u8; 4] = [0x5c, 0x11, 0xd7, 0x95];
const APPROVE: [u8; 4] = [0x09, 0x5e, 0xa7, 0xb3];

fn address(a: &[u8; 20]) -> [u8; 32] {
    let mut out = [0u8; 32];
    if let Some(tail) = out.get_mut(12..) {
        tail.copy_from_slice(a);
    }
    out
}

/// The words of a call whose one dynamic part is `path`, which goes last.
fn call(selector: [u8; 4], head: &[[u8; 32]], path: &[[u8; 20]]) -> Vec<u8> {
    let mut out = selector.to_vec();
    for w in head {
        out.extend_from_slice(w);
    }
    out.extend_from_slice(&word(u128::try_from(path.len()).unwrap_or(u128::MAX)));
    for a in path {
        out.extend_from_slice(&address(a));
    }
    out
}

/// Ether in: the amount goes as the transaction's value.
pub fn eth_for_tokens(min_out: u128, path: &[[u8; 20]], to: &[u8; 20], deadline: u64) -> Vec<u8> {
    let head = [word(min_out), word(0x80), address(to), word(u128::from(deadline))];
    call(ETH_FOR_TOKENS, &head, path)
}

/// A token in, for ether or another token out.
pub fn tokens_for(
    to_eth: bool,
    amount_in: u128,
    min_out: u128,
    path: &[[u8; 20]],
    to: &[u8; 20],
    deadline: u64,
) -> Vec<u8> {
    let selector = if to_eth { TOKENS_FOR_ETH } else { TOKENS_FOR_TOKENS };
    let head =
        [word(amount_in), word(min_out), word(0xa0), address(to), word(u128::from(deadline))];
    call(selector, &head, path)
}

/// `approve(spender, amount)`: the amount this swap takes and never more.
pub fn approve(spender: &[u8; 20], amount: u128) -> Vec<u8> {
    let mut out = APPROVE.to_vec();
    out.extend_from_slice(&address(spender));
    out.extend_from_slice(&word(amount));
    out
}

#[cfg(test)]
#[path = "calldata_test.rs"]
mod calldata_test;

//! The `settleBatch` call. A wallet settles one intent with no swap and no attestation, and one
//! sealed note per output. Pinned in the tests against Foundry's own encoder.

use super::abi::{bytes, bytes_array, tuple_of, word, words};

/// `settleBatch(bytes,uint256[],(address,uint64,uint64,uint256,uint256,address[],uint256),bytes,bytes[])`.
const SETTLE_BATCH: [u8; 4] = [0x7f, 0x47, 0x82, 0x16];
/// The residual's head is seven words, and its path starts right after them.
const PATH_OFFSET: u128 = 7 * 32;

/// The calldata settling `proof`, with `client_data` in output order.
pub fn settle_calldata(
    proof: &[u8],
    public_inputs: &[[u8; 32]],
    client_data: &[Vec<u8>],
) -> Vec<u8> {
    let parts = [
        bytes(proof),
        words(public_inputs),
        empty_residual(),
        bytes(&[]),
        bytes_array(client_data),
    ];
    let mut out = SETTLE_BATCH.to_vec();
    out.extend_from_slice(&tuple_of(&parts, None));
    out
}

/// No router, no assets, no amounts, an empty path and no deadline.
fn empty_residual() -> Vec<u8> {
    let mut out = Vec::with_capacity(8 * 32);
    for _ in 0..5 {
        out.extend_from_slice(&word(0));
    }
    out.extend_from_slice(&word(PATH_OFFSET));
    out.extend_from_slice(&word(0));
    out.extend_from_slice(&word(0));
    out
}

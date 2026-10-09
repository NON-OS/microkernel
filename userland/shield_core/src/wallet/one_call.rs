//! A v2 proof package in the verifier's single-call layout, the `proof` argument of `settleBatch`
//! when the wallet settles its own spend. The same cut as `SplitV2.whole` in the pool repository:
//! the body is split into head, claims and queries at the v2 sizes of the shape.

use super::abi::{bytes, word};
use sha3::{Digest, Keccak256};

const HEAD_A: usize = 3 * 32 + 4 + 4 + 88 * 16 + 4 + 3 * 32 + 4 + 256 * 16 + 4;
/// Eight query nonces, three fold nonces and the u32 row width.
const TAIL: usize = 8 * 8 + 3 * 8 + 4;
const FRI_PER_QUERY: usize = 16 * 24;
const ROW_PER_QUERY: usize = 8 * 44 + 16;
/// The committed periodic columns of every v2 shape.
const PERIODIC: usize = 59;
/// Shape A, which a phone proves, in the 37-limb and the 36-limb circuits, and its query count.
const SHAPE_A: [[u8; 4]; 2] = [[0xcc, 0xba, 0x76, 0xed], [0xad, 0xd1, 0x8d, 0xbb]];
const SHAPE_A_QUERIES: usize = 19;

/// The single-call proof for a format 7 package of shape A, or none for any other package.
pub fn one_call(package: &[u8]) -> Option<Vec<u8>> {
    let (header, body) = package.split_at_checked(40)?;
    if header.get(0..4)? != b"NOXP" || header.get(4..6)? != [7, 0] {
        return None;
    }
    let id = header.get(8..40)?;
    if !SHAPE_A.iter().any(|a| id.get(0..4) == Some(a.as_slice())) {
        return None;
    }
    let nq = SHAPE_A_QUERIES;
    let f = HEAD_A.checked_add(nq.checked_mul(FRI_PER_QUERY)?)?;
    let r = f.checked_add(TAIL)?.checked_add(nq.checked_mul(ROW_PER_QUERY)?)?;
    let c = r.checked_add(4)?.checked_add(PERIODIC.checked_mul(16)?)?.checked_add(8)?;
    if body.len() < c.checked_add(nq.checked_mul(8)?.checked_mul(PERIODIC)?)? {
        return None;
    }
    let head = [body.get(..HEAD_A)?, body.get(f..f.checked_add(TAIL)?)?].concat();
    let claims = body.get(r..c)?.to_vec();
    let tail = body.get(f.checked_add(TAIL)?..r)?;
    let queries = [body.get(HEAD_A..f)?, tail, body.get(c..)?].concat();
    Some(encode(&head, &claims, &queries, id))
}

/// `abi.encode(keccak256("NONOS-SHIELD-ONE-CALL-v1"), head, claims, queries, id, 0)`.
fn encode(head: &[u8], claims: &[u8], queries: &[u8], id: &[u8]) -> Vec<u8> {
    let parts = [bytes(head), bytes(claims), bytes(queries)];
    let mut out = Keccak256::digest(b"NONOS-SHIELD-ONE-CALL-v1").to_vec();
    let mut at: usize = 6 * 32;
    for part in &parts {
        out.extend_from_slice(&word(u128::try_from(at).unwrap_or(u128::MAX)));
        at = at.saturating_add(part.len());
    }
    let mut pid = [0u8; 32];
    pid.iter_mut().zip(id).for_each(|(slot, b)| *slot = *b);
    out.extend_from_slice(&pid);
    out.extend_from_slice(&word(0));
    parts.iter().for_each(|p| out.extend_from_slice(p));
    out
}

#[cfg(test)]
#[path = "one_call_test.rs"]
mod one_call_test;

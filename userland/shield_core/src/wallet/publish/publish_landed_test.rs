//! A spend lands only when both its nullifiers settle in one transaction.

use super::super::{landed, nullifiers, Seen};
use crate::net::rpc::RawLog;
use crate::prover::launch::publics::LIMBS;

fn spent(nf: [u8; 32], tx: u8) -> RawLog {
    RawLog { topics: vec![[0xaa; 32], nf], data: vec![], block: 9, tx: Some([tx; 32]) }
}
#[test]
fn a_spend_lands_only_when_both_nullifiers_settle_together() {
    let mut limbs = [0u64; LIMBS];
    limbs[8..16].copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
    let pair = nullifiers(&limbs);
    let mut first = [0u8; 32];
    for (i, limb) in [4u64, 3, 2, 1].iter().enumerate() {
        first[i * 8..i * 8 + 8].copy_from_slice(&limb.to_be_bytes());
    }
    assert_eq!(pair[0], first, "limb 0 is the lowest eight bytes of the word");
    let both = [spent(pair[0], 1), spent([9; 32], 2), spent(pair[1], 1)];
    assert_eq!(landed(&both, &pair), Seen::Landed(Some([1; 32])));
    assert_eq!(landed(&both[..2], &pair), Seen::Elsewhere, "one note went another way");
    assert_eq!(landed(&[spent(pair[0], 1), spent(pair[1], 3)], &pair), Seen::Elsewhere);
    assert_eq!(landed(&both[1..2], &pair), Seen::Neither);
}

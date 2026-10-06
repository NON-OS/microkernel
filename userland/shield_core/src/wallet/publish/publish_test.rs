// A test asserts by panicking, so the lints that forbid it are off here.
#![allow(clippy::expect_used, clippy::indexing_slicing, clippy::arithmetic_side_effects)]

use super::*;
use crate::net::rpc::RawLog;
use crate::prover::launch::publics::LIMBS;

const START: u64 = 1_790_841_600;

fn record() -> Published {
    Published {
        first: START,
        last: START,
        times: 1,
        route: Route::Lander,
        id: Some("h-7".into()),
        lander: 2,
    }
}

fn spent(nf: [u8; 32], tx: u8) -> RawLog {
    RawLog { topics: vec![[0xaa; 32], nf], data: vec![], block: 9, tx: Some([tx; 32]) }
}

#[test]
fn the_record_reads_back_and_refuses_anything_else() {
    let kept = record();
    assert_eq!(Published::from_text(&kept.to_text()), Some(kept));
    let bare = Published { id: None, ..record() };
    assert_eq!(Published::from_text(&bare.to_text()), Some(bare));
    // A record written before the list of landers reads as the first lander.
    let old = Published::from_text("1 2 3 lander h-7\n").expect("an old record");
    assert_eq!(old.lander, 0);
    for bad in
        ["", "1 2 3", "2 1 1 lander x", "1 2 3 relay x", "1 2 3 lander a/b", "1 2 3 lander x y"]
    {
        assert_eq!(Published::from_text(bad), None, "{bad}");
    }
}

/// Republished 15 minutes after the latest publication, and offered to the owner 30 minutes
/// after the first, never once both nullifiers are on chain.
#[test]
fn a_spend_waits_then_is_republished_then_offered_to_its_owner() {
    let r = record();
    assert_eq!(next(START + 60, &r, Seen::Neither), (Next::Wait, false));
    assert_eq!(next(START + REPUBLISH_AFTER, &r, Seen::Neither), (Next::Republish, false));
    let again = Published { last: START + REPUBLISH_AFTER, times: 2, ..record() };
    assert_eq!(next(START + SELF_SETTLE_AFTER - 1, &again, Seen::Neither), (Next::Wait, false));
    assert_eq!(next(START + SELF_SETTLE_AFTER, &again, Seen::Neither), (Next::Republish, true));
    let tx = Some([5; 32]);
    assert_eq!(next(START + 7200, &again, Seen::Landed(tx)), (Next::Landed(tx), false));
    assert_eq!(next(START + 7200, &again, Seen::Elsewhere), (Next::Elsewhere, false));
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

/// A spend its owner settled is never republished nor offered again, and lands as any other.
#[test]
fn a_spend_its_owner_settled_is_only_followed() {
    let r = record();
    assert_eq!(next_settling(START + REPUBLISH_AFTER, &r, Seen::Neither), (Next::Wait, false));
    assert_eq!(next_settling(START + SELF_SETTLE_AFTER, &r, Seen::Neither), (Next::Wait, false));
    let tx = Some([6; 32]);
    assert_eq!(next_settling(START + 60, &r, Seen::Landed(tx)), (Next::Landed(tx), false));
    assert_eq!(next_settling(START + 60, &r, Seen::Elsewhere), (Next::Elsewhere, false));
}

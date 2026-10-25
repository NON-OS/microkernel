use super::*;

#[path = "publish_landed_test.rs"]
mod landed;

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

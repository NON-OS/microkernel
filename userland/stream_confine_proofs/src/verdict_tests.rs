// NONOS Operating System (AGPL-3.0-or-later)
//! The verdict over every description of up to four streams, every unit,
//! and every choice of which unit is in service and which streams it
//! covers: unconfined only without `iommus`; attached only when the unit is
//! the one in service and covers every stream; refused otherwise.

use crate::hardware::broker::described::{DescribedIommus, MAX_DESCRIBED_STREAMS};
use crate::verdict::{verdict, Verdict};

#[test]
fn never_unconfined_behind_a_unit_and_attached_only_when_all_is_covered() {
    let ids = [0u32, 1, 2, 3];
    for count in 0..=MAX_DESCRIBED_STREAMS as u8 {
        for pick in 0..(4u32.pow(count as u32)) {
            let mut sids = [0u32; MAX_DESCRIBED_STREAMS];
            for (i, sid) in sids.iter_mut().take(count as usize).enumerate() {
                *sid = ids[(pick / 4u32.pow(i as u32) % 4) as usize];
            }
            for unit in [1u32, 2] {
                let d = DescribedIommus::Streams { unit, sids, count };
                for live in [None, Some(1u32), Some(2)] {
                    for covered_mask in 0..16u32 {
                        let covered = |sid: u32| covered_mask >> sid & 1 == 1;
                        let got = verdict(&d, |u| Some(u) == live, covered);
                        let all = d.sids().iter().all(|s| covered(*s));
                        let want_attach = live == Some(unit) && count > 0 && all;
                        assert_ne!(got, Verdict::Unconfined, "{d:?} left unconfined");
                        assert_eq!(
                            got == Verdict::Attach,
                            want_attach,
                            "{d:?} live {live:?} {covered_mask:b}"
                        );
                    }
                }
            }
        }
    }
    let never = |_: u32| panic!("asked about a unit with no iommus to name one");
    assert_eq!(verdict(&DescribedIommus::None, never, never), Verdict::Unconfined);
    let bad = DescribedIommus::Unreadable(b"x");
    assert_eq!(verdict(&bad, |_| true, |_| true), Verdict::Refuse(b"x"));
}

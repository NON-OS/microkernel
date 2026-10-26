// NONOS Operating System (AGPL-3.0-or-later)
//! Captured sound over seeded runs of batches, sound and then told one
//! lie: the driver's batch reader and ring give what the rule in
//! capture_sweep/model.rs gives at every take, every byte sent is kept,
//! counted lost or never seen, and a lie costs what it must.

use crate::batch::payloads;
use crate::capture_sweep::lie::{tell, Step, LIES};
use crate::capture_sweep::model::{self, Model};
use crate::capture_sweep::rng::{rounds, Rng};
use crate::ring::{Ring, RING_CAP};

#[test]
fn after_any_lie_capture_keeps_counts_and_orders_every_byte() {
    let mut told = vec![0u32; LIES.len()];
    for seed in 1..rounds(20_000) {
        let r = &mut Rng::new(seed);
        let lie = LIES[r.range(0, LIES.len() - 1)];
        let t = tell(r, lie);
        told[LIES.iter().position(|l| *l == lie).unwrap()] += 1;
        let at = format!("seed {seed}: {lie:?}");
        let (frame, share) = (t.rig.frame, t.rig.share);
        let mut ring = Ring::new(frame).sharing(share);
        let mut m = Model { q: Default::default(), lost: 0, frame, share };
        let (mut kept, mut lost) = (0, 0);
        for step in &t.steps {
            match step {
                Step::Batch(d) => {
                    payloads(d).for_each(|p| ring.put_packet(p));
                    model::payloads(d).into_iter().for_each(|p| m.packet(p));
                }
                Step::Take(most) => {
                    let got = ring.take(*most);
                    assert_eq!(got, m.take(*most), "{at}: the rule");
                    assert!(got.1.len().is_multiple_of(frame), "{at}: whole frames");
                    (kept, lost) = (kept + got.1.len(), lost + got.0 as usize);
                }
            }
        }
        assert_eq!(kept + lost + t.unseen, t.sent, "{at}: every byte accounted for");
        assert!(kept <= RING_CAP || t.lost.is_some(), "{at}");
        if let Some(want) = t.lost {
            assert_eq!(lost, want as usize, "{at}: what the lie costs");
        }
    }
    for (n, lie) in told.iter().zip(LIES) {
        assert!(cfg!(miri) || *n >= 2_000, "{lie:?} was told only {n} times");
    }
}

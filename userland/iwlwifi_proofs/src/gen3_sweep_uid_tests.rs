// NONOS Operating System (AGPL-3.0-or-later)
//! W4: each sweep takes the next of the firmware's four scan uids, so a late
//! completion of an earlier sweep ends nothing; a sweep given up as stalled
//! is aborted by its own uid, once.

use super::gen3_booted::booted;
use crate::gen3::sweep::{budget_ms, Sweep, Tick};
use crate::gen3_model::{Model, Out};

fn completion(uid: u32) -> Out {
    let mut p = vec![0u8; 16];
    p[0..4].copy_from_slice(&uid.to_le_bytes());
    p[6] = 1;
    Out { cmd: 0x0F, group: 0, seq: 0x8000, payload: p }
}

// The uids of every scan request (`cmd`, 0x0D) or abort (0x0E) sent.
fn uids(model: &Model, cmd: u8) -> Vec<u32> {
    let s = model.s.borrow();
    let of = |p: &[u8]| u32::from_le_bytes([p[0], p[1], p[2], p[3]]);
    s.commands.iter().filter(|c| (c.group, c.cmd) == (1, cmd)).map(|c| of(&c.payload)).collect()
}

#[test]
fn each_sweep_takes_the_next_uid_and_a_stale_completion_ends_nothing() {
    booted(|model, dev, c| {
        let mut sweep = Sweep::new();
        for (i, uid) in [0u32, 1, 2, 3, 0, 1].into_iter().enumerate() {
            assert_eq!(sweep.start(dev, c, &[1, 6], 0, &mut |_| {}), Ok(Tick::Running));
            if i > 0 {
                model.queue(completion((uid + 3) % 4));
                assert_eq!(sweep.pump(dev, 1, &mut |_| {}), Tick::Running, "the last sweep's");
            }
            model.queue(completion(uid));
            assert_eq!(sweep.pump(dev, 2, &mut |_| {}), Tick::Completed(1), "sweep {i}");
        }
        assert_eq!(uids(model, 0x0D), [0, 1, 2, 3, 0, 1], "four slots, in turn");
    });
}

#[test]
fn a_stalled_sweep_is_aborted_by_its_own_uid_once() {
    booted(|model, dev, c| {
        let mut sweep = Sweep::new();
        let past = u64::from(budget_ms(2)) + 1;
        assert_eq!(sweep.start(dev, c, &[1, 6], 0, &mut |_| {}), Ok(Tick::Running));
        assert_eq!(sweep.pump(dev, past, &mut |_| {}), Tick::Stalled);
        assert_eq!(sweep.abort_stalled(dev, c, &mut |_| {}), Ok(()));
        assert_eq!(sweep.abort_stalled(dev, c, &mut |_| {}), Ok(()), "nothing owed");
        assert_eq!(uids(model, 0x0E), [0], "aborted once, by its uid");
        assert_eq!(sweep.start(dev, c, &[1, 6], past, &mut |_| {}), Ok(Tick::Running));
        assert_eq!(sweep.abort(dev, c, &mut |_| {}), Ok(()));
        assert_eq!(uids(model, 0x0E), [0, 1], "the sweep in flight, by its own");
    });
}

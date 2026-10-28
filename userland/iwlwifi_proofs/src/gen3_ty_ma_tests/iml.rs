// NONOS Operating System (AGPL-3.0-or-later)

//! The image-loader wait of an integrated AX210-family part (SO, MA) is
//! Linux v6.12's `iwl_pcie_spin_for_iml`: checks back to back, never
//! sleeping, for up to 100 ms. It goes through `Clock::spin_for`, never
//! `poll_for` (which may sleep between checks); a discrete part (TY) does
//! not wait for the loader at all. `spin_until`, which the capsule's clock
//! runs it on, ends when ready, at its deadline or at its cap.

use super::rig::{polls, rig, run, GF, MA, MA_PNVM, MA_REV, TY, TY_PNVM, TY_REV};
use crate::gen3::region::{spin_until, Clock};
use crate::gen3_model::Polls;

// The model's clock, keeping the length of each sleepless wait asked of it.
struct Spins(Polls, Vec<u32>);

impl Clock for Spins {
    fn poll_for(&mut self, ms: u32, ready: &mut dyn FnMut() -> bool) -> bool {
        self.0.poll_for(ms, ready)
    }
    fn spin_for(&mut self, ms: u32, ready: &mut dyn FnMut() -> bool) -> bool {
        self.1.push(ms);
        self.0.spin_for(ms, ready)
    }
    fn delay_us(&mut self, us: u32) {
        self.0.delay_us(us)
    }
    fn now_ms(&mut self) -> u64 {
        self.0.now_ms()
    }
}

#[test]
fn an_integrated_part_spins_for_its_image_loader_and_a_discrete_one_does_not() {
    for (blob, pnvm, rev, discrete, want) in
        [(MA, MA_PNVM, MA_REV, false, vec![100]), (TY, TY_PNVM, TY_REV, true, vec![])]
    {
        let mut r = rig(blob, pnvm, rev, GF);
        r.board.discrete = discrete;
        let mut c = Spins(polls(), Vec::new());
        run(&r, &mut c).0.expect("alive");
        assert_eq!(c.1, want, "discrete: {discrete}");
    }
}

#[test]
fn a_sleepless_wait_ends_when_ready_at_its_deadline_or_at_its_cap() {
    let mut n = 0;
    let mut third = || {
        n += 1;
        n == 3
    };
    assert!(spin_until(&mut || 0, 100, 1000, &mut third));
    assert_eq!(n, 3, "ready on the third check");
    let (mut t, mut n) = (0u64, 0u32);
    let mut tick = || {
        t += 1;
        t
    };
    assert!(!spin_until(&mut tick, 100, 1_000_000, &mut || {
        n += 1;
        false
    }));
    assert_eq!(n, 100, "a millisecond a reading: 100 checks, then the deadline");
    let mut n = 0;
    assert!(!spin_until(&mut || 7, 100, 5000, &mut || {
        n += 1;
        false
    }));
    assert_eq!(n, 5000, "a clock that never moves: the cap ends it");
}

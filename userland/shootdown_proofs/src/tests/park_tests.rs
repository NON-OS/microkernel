// NONOS Operating System (AGPL-3.0-or-later)
//! The park path: an AP asked to park leaves the set only once reset, and it
//! parks only outside every lock.

use super::*;

#[test]
fn a_parked_ap_leaves_the_set_only_after_init() {
    // Asked to park at any point of bring-up, an AP is a target until it has
    // parked and INIT has reset it; the round never returns while it may
    // still run with the old entry.
    for n in PARK_CPUS {
        holds(&Config::kernel(n, 0, Flush::Page));
    }
}

#[test]
fn mutant_drop_at_park_request() {
    // Dropped at `AbandonRequested`, before `Parked`: the AP is still in
    // bring-up with whatever it cached, and the round returns past it.
    for n in PARK_CPUS {
        let cfg = Config { drop_after_init: false, ..Config::kernel(n, 0, Flush::Page) };
        assert_eq!(broken(&cfg), STALE);
    }
}

#[test]
fn every_checkpoint_of_bring_up_is_outside_every_lock() {
    use crate::model::{Step, BRING_UP};
    let mut held = false;
    for step in BRING_UP {
        match step {
            Step::Lock => held = true,
            Step::Unlock => held = false,
            Step::Checkpoint => assert!(!held, "a checkpoint inside a lock"),
        }
    }
    assert!(!held, "bring_up ends holding a lock");
    for n in PARK_CPUS {
        holds(&Config::kernel(n, 0, Flush::Page));
    }
}

#[test]
fn mutant_checkpoint_inside_a_lock() {
    use crate::model::Step;
    const MOVED: &[Step] =
        &[Step::Checkpoint, Step::Lock, Step::Checkpoint, Step::Unlock, Step::Checkpoint];
    for n in PARK_CPUS {
        let cfg = Config { bring_up: MOVED, ..Config::kernel(n, 0, Flush::Page) };
        assert_eq!(broken(&cfg), "an AP can park while holding a lock");
    }
}

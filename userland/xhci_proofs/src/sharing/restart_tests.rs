// NONOS Operating System (AGPL-3.0-or-later)
//! A sharer leaving a slot another keeps: its pipes lose their owner, so the
//! driver that takes its place configures them again, while the keeper's
//! pipes stay the keeper's.

use super::police_tests::{ask, composite_key, FIDO, HID};
use crate::protocol::*;
use crate::slots::Release;

/// The key's driver, started again under a new pid.
const FIDO_AGAIN: u32 = 21;

fn fido_leaves(s: &mut crate::slots::Sharing, alive: impl Fn(u32) -> bool) {
    assert_eq!(s.claims.release(4, FIDO, alive), Release::Kept);
    let c = &s.claims;
    s.owners.keep_holders(4, |p| c.holds(4, p));
}

#[test]
fn a_restarted_sharer_configures_its_pipes_again() {
    let mut s = composite_key();
    fido_leaves(&mut s, |_| true);
    s.claims.claim(4, 3, 1, FIDO_AGAIN).unwrap();
    for ep in [0x02, 0x82] {
        let ring = [4, ep, 64, 0, 5, 0];
        assert_eq!(ask(&mut s, FIDO_AGAIN, OP_ALLOC_TRANSFER_RING, &ring), Ok(()));
    }
    assert_eq!(ask(&mut s, FIDO_AGAIN, OP_INTERRUPT_OUT, &[4, 4, 1, 0, 0]), Ok(()));
    assert_eq!(ask(&mut s, HID, OP_INTERRUPT_OUT, &[4, 4, 1, 0, 0]), Err(-1), "the key's");
}

#[test]
fn the_keepers_pipes_stay_the_keepers() {
    let mut s = composite_key();
    fido_leaves(&mut s, |_| true);
    s.claims.claim(4, 3, 1, FIDO_AGAIN).unwrap();
    let ring = [4, 0x81, 8, 0, 10, 0];
    assert_eq!(ask(&mut s, FIDO_AGAIN, OP_ALLOC_TRANSFER_RING, &ring), Err(-16));
    assert_eq!(ask(&mut s, HID, OP_INTERRUPT_IN, &[4, 3, 8, 0]), Ok(()));
}

#[test]
fn the_departed_pid_is_refused_the_slot() {
    let mut s = composite_key();
    fido_leaves(&mut s, |_| true);
    assert_eq!(ask(&mut s, FIDO, OP_INTERRUPT_OUT, &[4, 4, 1, 0, 0]), Err(-1));
}

#[test]
fn a_dead_claimants_pipes_go_with_the_leaver() {
    const DEAD: u32 = 30;
    let mut s = composite_key();
    s.claims.claim(4, 3, 2, DEAD).unwrap();
    ask(&mut s, DEAD, OP_ALLOC_TRANSFER_RING, &[4, 0x83, 8, 0, 10, 0]).unwrap();
    fido_leaves(&mut s, |p| p != DEAD);
    s.claims.claim(4, 3, 1, FIDO_AGAIN).unwrap();
    let ring = [4, 0x83, 8, 0, 10, 0];
    assert_eq!(ask(&mut s, FIDO_AGAIN, OP_ALLOC_TRANSFER_RING, &ring), Ok(()));
}

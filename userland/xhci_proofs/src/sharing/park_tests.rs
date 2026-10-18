// NONOS Operating System (AGPL-3.0-or-later)
//! OP_PARK takes an addressed child out from under whoever drives it, so
//! only a driver admitted on both the child's slot and its hub's parks it:
//! a claimant of each, or its holder while it has none. Anyone else, or a
//! driver on only one of the two, gets -1.

use crate::slots::{gate_park, Sharing};

const HUB: u32 = 40;
const MSC: u32 = 50;
const STRANGER: u32 = 99;
const PARK: &str = include_str!("../../../capsule_driver_xhci/src/server/mux_park.rs");

/// usb_hub0 enabled the hub's slot 2 and its child's slot 5.
fn hub_and_child() -> Sharing {
    let mut s = Sharing::default();
    s.holders.set(2, HUB);
    s.holders.set(5, HUB);
    s
}

#[test]
fn only_the_driver_on_both_slots_parks() {
    let alive = |_| true;
    assert_eq!(gate_park(&mut hub_and_child(), 5, 2, HUB, alive), Ok(()));
    assert_eq!(gate_park(&mut hub_and_child(), 5, 2, STRANGER, alive), Err(-1));
    let mut s = hub_and_child();
    s.holders.set(5, MSC);
    assert_eq!(gate_park(&mut s, 5, 2, MSC, alive), Err(-1), "the child alone");
    assert_eq!(gate_park(&mut s, 5, 2, HUB, alive), Err(-1), "the hub alone");
    s.claims.claim(2, 1, 0, MSC).unwrap();
    assert_eq!(gate_park(&mut s, 5, 2, MSC, alive), Ok(()), "a claimant of the hub");
    let mut s = hub_and_child();
    s.claims.claim(2, 1, 0, MSC).unwrap();
    assert_eq!(gate_park(&mut s, 5, 2, HUB, alive), Err(-1), "the holder once it is shared");
}

#[test]
fn the_server_weighs_a_park_before_it_reads_one() {
    let gated = PARK.find("gate_park(&mut self.share, p.slot, p.parent, sender(), alive)");
    let addressed = PARK.find("self.addressed(p.slot)");
    assert!(gated.expect("parks are gated") < addressed.expect("then checked addressed"));
}

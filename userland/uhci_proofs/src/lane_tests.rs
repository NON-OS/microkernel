// NONOS Operating System (AGPL-3.0-or-later)
//! A root port's polled place is one slot's. No hub is enumerated, so two
//! slots on one port cannot arise on the wire; set up by hand, as a hub
//! would give it, the second is refused and never handed the place.

use crate::model::{addressed, bulk_ready, call, rig, Device};
use crate::wire::ops::{OP_BULK_IN_POLL, OP_ENABLE_SLOT};
use crate::wire::{E_AGAIN, E_BUSY};

fn poll(cx: &mut crate::model::Cx, s: u8) -> crate::model::Answer {
    call(cx, OP_BULK_IN_POLL, &[s, 0, 0, 2])
}

#[test]
fn a_second_slot_on_a_ports_place_is_refused_and_the_first_still_served() {
    let (mut cx, w) = rig([Some(Device::full(64)), None]);
    let a = addressed(&mut cx, 1);
    bulk_ready(&mut cx, a);
    assert_eq!(poll(&mut cx, a).status, E_AGAIN, "a arms on port 1");
    let b = call(&mut cx, OP_ENABLE_SLOT, &[]).data[0];
    let mut twin = *cx.table.dev(a).unwrap();
    (twin.poll, twin.bulk, twin.claimed) = (None, None, false);
    cx.table.devs[b as usize - 1] = Some(twin);
    bulk_ready(&mut cx, b);
    assert_eq!(poll(&mut cx, b).status, E_BUSY, "port 1's place is a's");
    w.borrow_mut().devs[0].as_mut().unwrap().send(&[6; 11]);
    crate::env::Clock::idle(&cx.clock);
    assert_eq!(poll(&mut cx, a).data[4..], [6; 11], "a served, its place untouched");
    assert_eq!(poll(&mut cx, b).status, E_AGAIN, "free again, b may arm");
}

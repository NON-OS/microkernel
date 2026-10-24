// NONOS Operating System (AGPL-3.0-or-later)
//! Polled places are held by devices, not root ports: two adapters below
//! one hub poll side by side, each into its own place; with every place
//! held a further device's first poll is E_BUSY, until one is freed.

use crate::devices::table::Dev;
use crate::hub_fixture::tree;
use crate::hub_seen::child;
use crate::model::hub::HubModel;
use crate::model::{addressed, bulk_ready, call, high, rig, Answer, Cx, Device};
use crate::sched::layout::POLL_PLACES;
use crate::wire::ops::OP_BULK_IN_POLL;
use crate::wire::{E_AGAIN, E_BUSY};

fn poll(cx: &mut Cx, s: u8) -> Answer {
    call(cx, OP_BULK_IN_POLL, &[s, 0, 0, 2])
}

#[test]
fn two_adapters_below_one_hub_poll_side_by_side() {
    let hub = HubModel::new(true, 4).plug(1, Device::full(64), 3).plug(2, Device::full(64), 3);
    let (mut cx, w, _, seen) = tree(hub);
    let (a, b) = (seen.bound[0].0, seen.bound[1].0);
    let port = |cx: &mut Cx, s: u8| cx.table.dev(s).unwrap().port;
    assert_eq!(port(&mut cx, a), port(&mut cx, b), "one root port");
    bulk_ready(&mut cx, a);
    bulk_ready(&mut cx, b);
    assert_eq!((poll(&mut cx, a).status, poll(&mut cx, b).status), (E_AGAIN, E_AGAIN));
    child(&w, 1).send(&[1; 20]);
    child(&w, 2).send(&[2; 30]);
    crate::env::Clock::idle(&cx.clock);
    assert_eq!(poll(&mut cx, b).data[4..], [2; 30]);
    assert_eq!(poll(&mut cx, a).data[4..], [1; 20]);
}

#[test]
fn with_every_place_held_a_first_poll_waits_for_one() {
    let (mut cx, _) = rig(vec![high(), None]);
    let a = addressed(&mut cx, 1);
    bulk_ready(&mut cx, a);
    for p in 0..POLL_PLACES {
        let holder = Dev { port: 2, poll: Some((p, 512)), ..Dev::default() };
        cx.table.devs[a as usize + p] = Some(holder);
    }
    assert_eq!(poll(&mut cx, a).status, E_BUSY, "no place free");
    cx.table.devs[a as usize + 3].as_mut().unwrap().poll = None;
    assert_eq!(poll(&mut cx, a).status, E_AGAIN, "a takes the freed place");
    assert_eq!(cx.table.dev(a).unwrap().poll.map(|(p, _)| p), Some(3));
}

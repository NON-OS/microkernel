// NONOS Operating System (AGPL-3.0-or-later)
//! Each root port has its own polled buffer: two network adapters poll side
//! by side, and one whose owner has gone holds back no other.

use crate::model::{addressed, bulk_ready, call, high, rig};
use crate::wire::ops::{OP_BULK_IN_POLL, OP_DISABLE_SLOT};
use crate::wire::E_AGAIN;

fn poll(cx: &mut crate::model::Cx, s: u8) -> crate::model::Answer {
    call(cx, OP_BULK_IN_POLL, &[s, 0, 0, 2])
}

#[test]
fn two_adapters_polling_at_once_each_get_their_own_frames() {
    let (mut cx, w) = rig(vec![high(), high(), None]);
    let (a, b) = (addressed(&mut cx, 1), addressed(&mut cx, 2));
    bulk_ready(&mut cx, a);
    bulk_ready(&mut cx, b);
    assert_eq!((poll(&mut cx, a).status, poll(&mut cx, b).status), (E_AGAIN, E_AGAIN));
    for round in 0..3u8 {
        w.borrow_mut().devs[0].as_mut().unwrap().send(&[round; 70]);
        w.borrow_mut().devs[1].as_mut().unwrap().send(&[0x80 | round; 9]);
        crate::env::Clock::idle(&cx.clock);
        let (ra, rb) = (poll(&mut cx, a), poll(&mut cx, b));
        assert_eq!((ra.status, &ra.data[4..]), (0, &[round; 70][..]), "a, round {round}");
        assert_eq!((rb.status, &rb.data[4..]), (0, &[0x80 | round; 9][..]), "b, round {round}");
        assert_eq!((poll(&mut cx, a).status, poll(&mut cx, b).status), (E_AGAIN, E_AGAIN));
    }
    let w = w.borrow();
    assert!(w.devs.iter().flatten().all(|d| d.toggle_errors == 0));
}

#[test]
fn an_owner_that_never_polls_again_holds_back_no_other_and_its_slot_frees_its_buffer() {
    let (mut cx, w) = rig(vec![high(), high(), None]);
    let (a, b) = (addressed(&mut cx, 1), addressed(&mut cx, 2));
    bulk_ready(&mut cx, a);
    bulk_ready(&mut cx, b);
    assert_eq!(poll(&mut cx, a).status, E_AGAIN, "a arms, then its owner dies");
    w.borrow_mut().devs[0].as_mut().unwrap().send(&[1; 30]);
    assert_eq!(poll(&mut cx, b).status, E_AGAIN, "b arms");
    for i in 0..5u8 {
        w.borrow_mut().devs[1].as_mut().unwrap().send(&[i; 12]);
        crate::env::Clock::idle(&cx.clock);
        assert_eq!(poll(&mut cx, b).data[4..], [i; 12], "b served, round {i}");
        assert_eq!(poll(&mut cx, b).status, E_AGAIN);
    }
    assert_eq!(call(&mut cx, OP_DISABLE_SLOT, &[a]).status, 0, "the kernel closes a's slot");
    assert!(cx.table.dev(a).is_none());
    let again = addressed(&mut cx, 1);
    bulk_ready(&mut cx, again);
    w.borrow_mut().devs[0].as_mut().unwrap().send(&[7; 5]);
    assert_eq!(poll(&mut cx, again).status, E_AGAIN, "a fresh qTD");
    crate::env::Clock::idle(&cx.clock);
    assert_eq!(poll(&mut cx, again).data[4..], [7; 5]);
}

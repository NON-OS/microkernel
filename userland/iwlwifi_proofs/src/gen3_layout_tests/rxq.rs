// NONOS Operating System (AGPL-3.0-or-later)
//! The receive queue posts every buffer, and a taken buffer is checked
//! against the ones posted.

use crate::gen3::plan::*;
use crate::gen3::region::Region;
use crate::gen3::rxq::{RxQueue, Taken};
use crate::gen3_model::{Mem, Model};
use super::regions::regions;

#[test]
fn the_receive_queue_posts_every_buffer_and_tells_the_device_in_eights() {
    let (ctrl, rbs) = regions();
    let model = Model::new(ctrl.clone(), rbs.clone());
    let mut q = RxQueue::new();
    assert!(q.init(ctrl.as_ref(), rbs.as_ref()));
    let mut d = [0u8; 16];
    ctrl.read(RX_FREE + 5 * 16, &mut d);
    assert_eq!(u16::from_le_bytes([d[0], d[1]]), 6, "buffer id, 1-based");
    assert_eq!(u64::from_le_bytes(d[8..16].try_into().unwrap()), rbs.dev() + 5 * 2048);
    q.kick(&model);
    assert_eq!(model.s.borrow().widx, 64);
    model.s.borrow_mut().widx = 0;
    q.kick(&model);
    assert_eq!(model.s.borrow().widx, 0, "an unchanged index is not written again");
}

// Put a completion for buffer `vid` at used index `i` and close up to `closed`.
fn complete_at(ctrl: &Mem, i: usize, vid: u16, flags: u8, closed: u16) {
    let mut cd = [0u8; 32];
    cd[4..6].copy_from_slice(&vid.to_le_bytes());
    cd[6] = flags;
    ctrl.write(RX_USED + i * 32, &cd);
    ctrl.write(RB_STTS, &closed.to_le_bytes());
}

#[test]
fn taking_a_buffer_checks_the_device_named_one_we_posted() {
    let (ctrl, rbs) = regions();
    let mut q = RxQueue::new();
    q.init(ctrl.as_ref(), rbs.as_ref());
    rbs.write(2 * 2048, b"payload");
    let mut out = [0u8; 2048];
    assert_eq!(q.take(ctrl.as_ref(), rbs.as_ref(), &mut out), None, "nothing closed yet");
    complete_at(&ctrl, 0, 3, 0, 1);
    assert_eq!(q.take(ctrl.as_ref(), rbs.as_ref(), &mut out), Some(Taken::Buffer));
    assert_eq!(&out[..7], b"payload");
    let mut d = [0u8; 16];
    ctrl.read(RX_FREE + 64 * 16, &mut d);
    assert_eq!(u16::from_le_bytes([d[0], d[1]]), 3, "the buffer went straight back on the ring");
    complete_at(&ctrl, 1, 3, 0, 2);
    complete_at(&ctrl, 2, 0, 0, 3);
    complete_at(&ctrl, 3, 65, 0, 4);
    assert_eq!(q.take(ctrl.as_ref(), rbs.as_ref(), &mut out), Some(Taken::Buffer), "re-posted id is fine");
    assert_eq!(q.take(ctrl.as_ref(), rbs.as_ref(), &mut out), Some(Taken::BadId), "id zero");
    assert_eq!(q.take(ctrl.as_ref(), rbs.as_ref(), &mut out), Some(Taken::BadId), "id past the pool");
    complete_at(&ctrl, 4, 9, 1, 5);
    complete_at(&ctrl, 5, 10, 0, 6);
    assert_eq!(q.take(ctrl.as_ref(), rbs.as_ref(), &mut out), Some(Taken::Fragment));
    assert_eq!(q.take(ctrl.as_ref(), rbs.as_ref(), &mut out), Some(Taken::Fragment), "the rest of that frame");
    // A closed index wider than the ring is masked to it.
    complete_at(&ctrl, 6, 11, 0, 0x1000 | 7);
    assert_eq!(q.take(ctrl.as_ref(), rbs.as_ref(), &mut out), Some(Taken::Buffer));
}

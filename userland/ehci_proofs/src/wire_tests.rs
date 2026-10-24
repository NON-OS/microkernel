// NONOS Operating System (AGPL-3.0-or-later)
//! The wire: replies echo the request, and what is not served is refused.

use crate::model::{call, rig};
use crate::wire::ops::*;
use crate::wire::E_INVAL;

#[test]
fn a_healthcheck_answers_zero_and_echoes_the_request_id() {
    let (mut cx, _) = rig(vec![None, None]);
    let r = call(&mut cx, OP_HEALTHCHECK, &[]);
    assert_eq!((r.status, r.request_id, r.data.len()), (0, 0x5A5A, 0));
}

#[test]
fn hub_isochronous_and_unknown_ops_are_e_inval() {
    let (mut cx, _) = rig(vec![None, None]);
    for op in [0x18u16, 0x19, 0x1A, 0x20, 0x21, 0x02, 0xFFFF] {
        assert_eq!(call(&mut cx, op, &[1, 2, 3, 4]).status, E_INVAL, "op {op:#x}");
    }
}

#[test]
fn sixteen_slots_then_busy() {
    let (mut cx, _) = rig(vec![None, None]);
    for want in 1..=16u8 {
        assert_eq!(call(&mut cx, OP_ENABLE_SLOT, &[]).data[0], want);
    }
    assert_eq!(call(&mut cx, OP_ENABLE_SLOT, &[]).status, crate::wire::E_BUSY);
}

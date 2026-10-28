// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! The shipped schedule region (platform/region.rs) over host memory: an
//! access past its end is refused and told once, never made and never a
//! panic, and the request that made one is answered E_IO (dispatch.rs).

#[path = "../../capsule_driver_ohci/src/platform/region.rs"]
mod region;
#[path = "../../capsule_driver_ohci/src/platform/span.rs"]
mod span;

use crate::env::{Mem, OutOfRegion};
use crate::model::rig;
use crate::sched::layout::REGION;
use crate::server::context::Ctx;
use crate::server::dispatch::dispatch;
use crate::wire::ops::OP_HEALTHCHECK;
use crate::wire::{Request, E_IO, HDR_LEN, REPLY_MAX, STATUS_LEN};
use nonos_libc::DmaMapOut;
use region::Region;

fn region(buf: &mut [u8]) -> Region {
    let (user_va, length) = (buf.as_mut_ptr() as u64, REGION as u64);
    let map = DmaMapOut { user_va, device_addr: 0x10_0000, length, grant_id: 1 };
    Region::new(&map).expect("a region below 4 GiB")
}

#[test]
fn an_access_past_the_region_is_refused_and_told_once_never_made() {
    let mut buf = vec![0u8; REGION + 8];
    let r = region(&mut buf);
    r.wr(REGION - 4, 0x1234_5678);
    assert_eq!((r.rd(REGION - 4), r.fault()), (0x1234_5678, Ok(())));
    r.wr(REGION - 2, 0xAAAA_AAAA);
    assert_eq!(r.rd(REGION), !0, "past the end reads all ones");
    assert_eq!((r.fault(), r.fault()), (Err(OutOfRegion), Ok(())), "told once");
    let mut out = [0u8; 8];
    r.read_bytes(REGION - 4, &mut out);
    r.write_bytes(usize::MAX, &[1]);
    assert_eq!((out, r.fault()), ([0xFF; 8], Err(OutOfRegion)));
    assert_eq!(&buf[REGION - 2..], &[0x34, 0x12, 0, 0, 0, 0, 0, 0, 0, 0], "nothing past it");
}

#[test]
fn a_request_that_reached_past_the_region_is_answered_e_io() {
    let mut buf = vec![0u8; REGION];
    let (model, _w) = rig(vec![None, None, None]);
    let mut cx = Ctx::new(model.io, model.clock, region(&mut buf));
    let req = Request { op: OP_HEALTHCHECK, flags: 0, request_id: 9, payload_len: 0 };
    let mut tx = vec![0u8; HDR_LEN + STATUS_LEN + REPLY_MAX];
    let status = |tx: &[u8]| i32::from_le_bytes(tx[HDR_LEN..HDR_LEN + 4].try_into().unwrap());
    cx.mem.wr(REGION, 1);
    dispatch(&mut cx, &req, &[], &mut tx);
    assert_eq!(status(&tx), E_IO);
    dispatch(&mut cx, &req, &[], &mut tx);
    assert_eq!(status(&tx), 0, "the next is served");
}

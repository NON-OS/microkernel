// NONOS Operating System (AGPL-3.0-or-later)

//! The RTL8822CE MAC init program, as an operation table for the shared
//! executor: rtw88 `rtw8822c_mac_init` (rtw8822c.c:2004, Linux v6.12) in
//! three parts, then the shared `rtw_drv_info_cfg` tail (mac.c:1364). The
//! register offsets are reg.h's and bf.h's, the values the `WLAN_*` defines
//! of rtw8822c.c, each composite one worked out in a comment. Register facts
//! from "GPL-2.0 OR BSD-3-Clause" files, used under BSD-3-Clause.
//!
//! rtw88 writes a few registers read-modify-write in one access; the table
//! has no such step, so those are a clear and then a set of the same register
//! (`clear16`, `set16`), which leaves the value rtw88 leaves.

mod edca;
mod protocol;
mod wmac;

use super::op::{Kind, MacOp, Width};

/// The whole program: protocol, EDCA and timing, WMAC and the tail.
pub const MAC_INIT: &[MacOp] =
    &join::<{ protocol::LEN }, { edca::LEN + wmac::LEN }, { MAC_LEN }>(protocol::OPS, &REST);

const MAC_LEN: usize = protocol::LEN + edca::LEN + wmac::LEN;
const REST: [MacOp; edca::LEN + wmac::LEN] =
    join::<{ edca::LEN }, { wmac::LEN }, { edca::LEN + wmac::LEN }>(edca::OPS, wmac::OPS);

/// `a` then `b`, as one array of `C = A + B`.
const fn join<const A: usize, const B: usize, const C: usize>(
    a: &[MacOp; A],
    b: &[MacOp; B],
) -> [MacOp; C] {
    assert!(A + B == C);
    let mut out = [MacOp::write8(0, 0); C];
    let mut i = 0;
    while i < C {
        out[i] = if i < A { a[i] } else { b[i - A] };
        i += 1;
    }
    out
}

pub(super) const fn clear16(offset: u16, value: u16) -> MacOp {
    MacOp { offset, width: Width::B16, kind: Kind::Clear, value: value as u32 }
}

pub(super) const fn set16(offset: u16, value: u16) -> MacOp {
    MacOp { offset, width: Width::B16, kind: Kind::Set, value: value as u32 }
}

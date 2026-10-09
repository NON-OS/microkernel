// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The doorbells sit at offsets the device steers through CAP.DSTRD, and the
//! broker may map less of BAR0 than the BAR's size (it stops below an MSI-X
//! table that shares the BAR). The driver takes a controller only when every
//! doorbell it rings lies inside the window it was given.

use crate::admin::admin_doorbells;
use crate::controller::ControllerInfo;
use crate::nvm::{cq_head_doorbell, sq_tail_doorbell, IO_QID};

pub(crate) fn info(cap: u64) -> ControllerInfo {
    ControllerInfo {
        cap,
        version: 0x0001_0400,
        cc: 0,
        csts: 0,
        aqa: 0,
        intms: 0,
        intmc: 0,
        cmbloc: 0,
        cmbsz: 0,
    }
}

pub(crate) fn cap_with_stride(cap: u64, stride: u8) -> u64 {
    (cap & !(0xf << 32)) | ((stride as u64 & 0xf) << 32)
}

/// The spec's doorbell offset, written out apart from the driver's.
fn spec_doorbell(qid: u64, completion: bool, stride: u8) -> u64 {
    0x1000 + (2 * qid + completion as u64) * (4u64 << stride)
}

/// Every doorbell the driver writes, as (offset, what it is).
fn rung(stride: u8) -> [(u64, &'static str); 4] {
    let (sq0, cq0) = admin_doorbells(stride);
    [
        (sq0 as u64, "admin submission tail"),
        (cq0 as u64, "admin completion head"),
        (sq_tail_doorbell(IO_QID, stride) as u64, "I/O submission tail"),
        (cq_head_doorbell(IO_QID, stride) as u64, "I/O completion head"),
    ]
}

#[test]
fn the_driver_rings_the_doorbells_the_spec_places() {
    for stride in 0..16u8 {
        let [sq0, cq0, sq1, cq1] = rung(stride);
        assert_eq!(sq0.0, spec_doorbell(0, false, stride));
        assert_eq!(cq0.0, spec_doorbell(0, true, stride));
        assert_eq!(sq1.0, spec_doorbell(IO_QID as u64, false, stride));
        assert_eq!(cq1.0, spec_doorbell(IO_QID as u64, true, stride));
    }
}

#[test]
fn an_accepted_window_holds_every_doorbell_the_driver_rings() {
    let mut s = 0x9e37_79b9_7f4a_7c15u64;
    for stride in 0..16u8 {
        for mapped in (0..=0x70000u64).step_by(4).chain([u64::MAX, u64::MAX - 3]) {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            let ok = info(cap_with_stride(s, stride)).doorbells_fit(mapped);
            let all_inside = rung(stride).iter().all(|(off, _)| off + 4 <= mapped);
            assert_eq!(ok, all_inside, "stride {stride} window {mapped:#x}");
        }
    }
}

#[test]
fn a_stride_or_window_the_driver_cannot_reach_is_refused() {
    // (what, CAP.DSTRD, mapped bytes of BAR0, taken)
    let cases: [(&str, u8, u64, bool); 9] = [
        ("QEMU layout: MSI-X table at 0x2000, stride 0", 0, 0x2000, true),
        ("MSI-X table at 0x1000 leaves no doorbell page", 0, 0x1000, false),
        ("16 KiB BAR, stride 0", 0, 0x4000, true),
        ("16 KiB BAR, stride 10: I/O head lands at 0x4000", 10, 0x4000, false),
        ("16 KiB BAR, stride 9: I/O head at 0x2800", 9, 0x4000, true),
        ("16 KiB BAR, the widest stride", 15, 0x4000, false),
        ("the widest stride in a window that just holds it", 15, 0x61004, true),
        ("one byte short of the I/O head doorbell", 15, 0x61003, false),
        ("an all-ones CAP in a 64 KiB window", 15, 0x10000, false),
    ];
    for (what, stride, mapped, taken) in cases {
        let cap = if what.contains("all-ones") { u64::MAX } else { cap_with_stride(0, stride) };
        assert_eq!(info(cap).doorbells_fit(mapped), taken, "{what}");
    }
}

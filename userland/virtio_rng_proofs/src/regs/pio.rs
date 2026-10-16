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

//! Port IO, which the host has no ports for: the legacy device in `port`
//! answers behind the grant. It holds the status byte and the request
//! queue; any other register panics rather than answer, so no test reads a
//! fabricated device response as if it were real. A window built with
//! `Regs::mmio` never reaches here.

use super::port::{port, Status};
use crate::constants::{LEG_GUEST_FEATURES, LEG_HOST_FEATURES, LEG_STATUS};
use crate::constants::{LEG_QUEUE_NUM, LEG_QUEUE_PFN, LEG_QUEUE_SEL};

pub fn read(_grant: u64, offset: usize, _width: u8) -> u32 {
    port(|p| match offset {
        LEG_STATUS => {
            let status = if p.held > 0 { p.stale } else { p.status };
            p.held = p.held.saturating_sub(1);
            p.log.push(Status::Read(status));
            u32::from(status)
        }
        LEG_HOST_FEATURES => 0,
        LEG_QUEUE_NUM => u32::from(if p.sel == 0 { p.qnum } else { 0 }),
        _ => panic!("virtio_rng_proofs: the port device holds no register at {offset:#x}"),
    })
}

pub fn write(_grant: u64, offset: usize, _width: u8, value: u32) {
    port(|p| match offset {
        LEG_STATUS => {
            if value == 0 {
                p.stale = p.status;
            }
            p.status = value as u8;
            p.log.push(Status::Write(p.status));
        }
        LEG_GUEST_FEATURES => {}
        LEG_QUEUE_SEL => p.sel = value as u16,
        LEG_QUEUE_PFN => p.pfn = value,
        _ => panic!("virtio_rng_proofs: the port device holds no register at {offset:#x}"),
    })
}

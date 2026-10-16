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

//! "Seen stopped" is both answers: HAIMR must take the DMA reset and the
//! ring flush that follow the two stops (engine::stopped). A chip that
//! leaves either untaken never has bus mastering turned on, and neither
//! does a claim whose first command write the broker refuses; the reader
//! is let go with the release last.

use nonos_libc::{refuse_command, violations};

use super::claim_tests::refused;
use super::rig::knobs;
use crate::error::RtsxError;
use crate::model::{left_armed, Knobs};
use crate::regs::pm::{DMACTL, RBCTL};

fn untaken(reg: u16) {
    left_armed();
    knobs(Knobs { untaken: Some(reg), ..Knobs::default() });
    let ev = refused(RtsxError::RegisterMismatch);
    assert_eq!(violations(), 0, "{ev:?}");
}

#[test]
fn a_dma_reset_the_chip_does_not_take_keeps_bus_mastering_off() {
    untaken(DMACTL);
}

#[test]
fn a_ring_flush_the_chip_does_not_take_keeps_bus_mastering_off() {
    untaken(RBCTL);
}

#[test]
fn a_refused_claim_time_command_write_ends_the_claim_there() {
    refuse_command();
    let ev = refused(RtsxError::PciCommand);
    let want = ["claim 0x52270001", "mmio_map", "pci 0x4=0x0402 refused", "release 0x52270001"];
    assert_eq!(ev, want, "only the release after the refused write");
}

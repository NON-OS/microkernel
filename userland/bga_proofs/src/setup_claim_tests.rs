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

//! What one bring-up attempt asks of the PCI function and of the adapter
//! before it sets a mode, and what its handles give back.
//!
//! The capsule used to write Bus Master as the whole command word at claim,
//! though the adapter does no DMA, and to program any function behind the
//! Bochs PCI ID without asking the DISPI interface what it is.

use nonos_libc::{command, given_back, MK_PCI_CMD_MEMORY_SPACE, PCI_CMD_IO_SPACE};

use crate::adapter::{adapter, enabled, found, DEVICE, MODE_BYTES, REG_LEN};
use crate::constants::{CLEAR_COLOR, DISPI_INDEX_ID};
use crate::dispi::dispi_off;
use crate::error::BgaError;
use crate::setup::run;

#[test]
fn the_adapter_is_claimed_with_memory_decoding_on_and_bus_mastering_off() {
    let _a = adapter(REG_LEN, MODE_BYTES);
    assert!(run(found(MODE_BYTES)).is_ok(), "the attempt failed");
    assert_eq!(command(), PCI_CMD_IO_SPACE | MK_PCI_CMD_MEMORY_SPACE, "{:#06x}", command());
}

#[test]
fn a_function_that_is_no_dispi_adapter_is_refused_before_a_mode_is_written() {
    for id in [0xffff, 0xB0C6, 0] {
        let a = adapter(REG_LEN, MODE_BYTES);
        a.regs.present16(dispi_off(DISPI_INDEX_ID) as usize, id);
        assert_eq!(run(found(MODE_BYTES)).err(), Some(BgaError::NotDispi), "{id:#x}");
        assert_eq!(enabled(&a), 0, "a mode was programmed on {id:#x}");
        assert_eq!(a.fb.wrote32(0), 0, "the framebuffer was cleared on {id:#x}");
    }
    assert_eq!(given_back().iter().filter(|g| g.0 == "device_release").count(), 3);
}

#[test]
fn the_drivers_handles_give_back_both_mappings_and_then_the_claim() {
    let a = adapter(REG_LEN, MODE_BYTES);
    let d = run(found(MODE_BYTES)).unwrap_or_else(|e| panic!("refused: {e:?}"));
    assert_eq!(a.fb.wrote32(0), CLEAR_COLOR);
    let before = given_back().len();
    drop(d.handles);
    let back: Vec<&str> = given_back()[before..].iter().map(|g| g.0).collect();
    assert_eq!(back, ["mmio_unmap", "mmio_unmap", "device_release"]);
    assert_eq!(given_back().last(), Some(&("device_release", DEVICE)));
}

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

//! The other edges of one bring-up attempt: a framebuffer mapping one pixel
//! short of the mode, a register mapping that ends before the DISPI block,
//! and a short BAR as discovery reports it.

use nonos_libc::{offer, Bar, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

use crate::adapter::{adapter, enabled, found, DEVICE, GUARD, MODE_BYTES, REG_LEN};
use crate::constants::{DEVICE_BGA, FB_BAR, PCI_CLASS_DISPLAY, REG_BAR, VENDOR_QEMU_BOCHS};
use crate::discover::find_bga;
use crate::dispi::regs_len;
use crate::error::BgaError;
use crate::setup::run;

#[test]
fn a_mapping_one_pixel_short_of_the_mode_is_refused() {
    let a = adapter(REG_LEN, MODE_BYTES - 4);
    assert_eq!(run(found(MODE_BYTES - 4)).err(), Some(BgaError::ModeTooLarge));
    assert_eq!(a.fb.wrote32(MODE_BYTES - 4), GUARD, "the clear ran past the end of the mapping");
    assert_eq!(enabled(&a), 0, "the refused mode was programmed anyway");
}

#[test]
fn a_register_mapping_that_ends_before_dispi_enable_is_refused() {
    let short = regs_len() as usize - 1;
    let a = adapter(short, MODE_BYTES);
    assert_eq!(run(found(MODE_BYTES)).err(), Some(BgaError::RegisterWindowTooSmall));
    assert_eq!(a.regs.wrote32(short), GUARD, "a register past the mapping was written");
}

#[test]
fn a_short_framebuffer_bar_from_discovery_is_refused_end_to_end() {
    let mut r = DeviceRecord { device_id: DEVICE, bus_kind: BUS_KIND_PCI, ..Default::default() };
    (r.pci_class, r.vendor, r.device, r.bar_count) =
        (PCI_CLASS_DISPLAY, VENDOR_QEMU_BOCHS, DEVICE_BGA, 3);
    r.bars[FB_BAR as usize] = Bar { size: 2 << 20, kind: BAR_KIND_MMIO };
    r.bars[REG_BAR as usize] = Bar { size: REG_LEN as u64, kind: BAR_KIND_MMIO };
    offer(vec![r]);
    let dev = find_bga().expect("the adapter was not found");
    let a = adapter(REG_LEN, dev.fb_size as usize);
    assert_eq!(run(dev).err(), Some(BgaError::ModeTooLarge));
    assert_eq!(a.fb.wrote32(2 << 20), GUARD, "the clear ran past the end of the mapping");
}

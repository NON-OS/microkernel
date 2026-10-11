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

//! The bring-up against a controller as QEMU and VirtualBox present it.

use nonos_libc::{calls, MK_DMA_MAP_COHERENT, MK_DMA_MAP_DMA32};

use super::util::{up, DEVICE};
use crate::model::{EXT_CTRL, FRONT_RATE, HEADPHONE, MASTER, PCM_OUT};
use crate::setup::claim::COMMAND_BITS;

#[test]
fn the_grants_are_taken_in_order_and_kept() {
    let (_d, _ich) = up();
    let flags = (MK_DMA_MAP_DMA32 | MK_DMA_MAP_COHERENT) as u64;
    assert_eq!(
        calls(),
        vec![
            ("claim", DEVICE),
            ("pci_command", COMMAND_BITS as u64),
            ("pio_grant", 0),
            ("pio_grant", 1),
            ("dma_map", flags),
        ]
    );
}

#[test]
fn the_link_is_cold_reset_with_interrupts_off() {
    let (_d, ich) = up();
    let ich = ich.borrow();
    assert!(ich.cold_cycled, "cold reset asserted, then released");
    assert_eq!(ich.glob_cnt & 0x0030_000B, 0x2, "two channels, link on, no GIE, out of reset");
}

#[test]
fn the_codec_comes_up_at_48k_with_every_output_muted() {
    let (d, ich) = up();
    let ich = ich.borrow();
    assert!(ich.writes.contains(&(50, 0x00, 0)), "the codec was reset");
    assert_eq!(ich.mixer[EXT_CTRL] & 1, 0, "variable rate off");
    assert_eq!(ich.mixer[FRONT_RATE], 48_000);
    for reg in [MASTER, HEADPHONE, PCM_OUT] {
        assert_ne!(ich.mixer[reg] & 0x8000, 0, "register {reg} muted while nothing plays");
    }
    assert_eq!((d.ident.vendor, d.ident.device), (0x8384, 0x7600));
}

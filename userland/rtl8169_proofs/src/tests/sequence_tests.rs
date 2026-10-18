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

//! The setup sequence on the claim it is handed. The kernel's boot pass
//! leaves Bus Master on; the sequence writes the Command word with Memory
//! Space alone before it maps the BAR or any DMA frame, so a part an earlier
//! owner left receiving cannot reach memory before the bring-up resets it.

use std::sync::{atomic::AtomicU32, Arc};

use nonos_libc::{command, map_window, seen, set_command, watch, Step};
use nonos_libc::{MK_PCI_CMD_BUS_MASTER as BM, MK_PCI_CMD_MEMORY_SPACE as MEM};

use super::memory::CHIP;
use super::model::window;
use super::probe::probe;
use crate::constants::regs::REG_TX_CONFIG;
use crate::discover::Found;
use crate::setup::run;

#[test]
fn the_claim_is_written_bus_master_off_before_any_grant() {
    let bar = window();
    bar.present32(REG_TX_CONFIG, 0x541 << 20);
    map_window(bar.base(), bar.len() as u64);
    set_command(MEM | BM);
    watch(probe(&bar, &Arc::new(AtomicU32::new(0))));
    let found = Found { device_id: 1, pci_device: 0x8168, bar_index: 2, bar_size: 0x100 };
    let d = run(found).expect("set up");
    let steps: Vec<Step> = seen().iter().map(|s| s.step).collect();
    assert_eq!(steps, [Step::Command(MEM)], "one write, Memory Space alone");
    assert_eq!(command() & BM, 0, "still off when the bring-up starts");
    assert_eq!(d.chip.ver, CHIP.ver, "the chip read through the mapped BAR");
}

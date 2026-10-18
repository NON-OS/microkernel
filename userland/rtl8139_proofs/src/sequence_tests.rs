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
//! leaves Bus Master on; the sequence writes it off before it takes the port
//! grant or any DMA frame, so a part an earlier owner left receiving cannot
//! reach memory before the bring-up has reset it.

use nonos_libc::{command, line_up, seen, set_command, watch, DmaMapOut, Step};
use nonos_libc::{MK_PCI_CMD_BUS_MASTER as BM, MK_PCI_CMD_MEMORY_SPACE as MEM};

use crate::discover::Found;
use crate::part::{part_with, DEVICE};
use crate::probe::probe;
use crate::setup::run;

fn grant(device_addr: u64, length: u64, grant_id: u64) -> DmaMapOut {
    DmaMapOut { user_va: 0x10_0000, device_addr, length, grant_id }
}

#[test]
fn the_claim_is_written_bus_master_off_before_any_grant() {
    let (_port, resets) = part_with(false, None);
    set_command(MEM | BM);
    watch(probe(&resets));
    line_up(&[grant(0x10_0000, 0xB000, 4), grant(0x20_0000, 0x2000, 5)]);
    let d = run(Found { device_id: DEVICE, pio_bar_index: 0 }).expect("set up");
    let steps: Vec<Step> = seen().iter().map(|s| s.step).collect();
    assert_eq!(steps, [Step::Command(0)], "one write, Bus Master off");
    assert_eq!(command() & BM, 0, "still off when the bring-up starts");
    assert_eq!((d.rx_device_addr, d.tx_device_addr), (0x10_0000, 0x20_0000));
}

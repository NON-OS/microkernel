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

//! The shim's broker constants are the libc's: each is held to the line
//! that defines it in the libc source, which cannot be mounted here.

const LIBC: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../libc/src/broker/");

#[test]
fn the_shims_broker_constants_are_the_libcs() {
    let lines = [
        ("irq.rs", "pub const MK_IRQ_BIND_MSIX: u32 = 1 << 0;"),
        ("dma.rs", "pub const MK_DMA_MAP_COHERENT: u32 = 1 << 2;"),
        ("pci.rs", "pub const MK_PCI_CFG_COMMAND: u32 = 0x04;"),
        ("pci.rs", "pub const MK_PCI_CMD_MEMORY_SPACE: u16 = 1 << 1;"),
        ("pci.rs", "pub const MK_PCI_CMD_BUS_MASTER: u16 = 1 << 2;"),
        ("irq_wait.rs", "pub const MK_IRQ_WAIT_TIMED_OUT: i64 = 1;"),
    ];
    let shim = concat!(env!("CARGO_MANIFEST_DIR"), "/libc_shim/src/");
    let ours = ["consts.rs", "irq.rs"].map(|f| std::fs::read_to_string(format!("{shim}{f}")));
    let ours: String = ours.into_iter().map(|t| t.expect("shim source")).collect();
    for (file, line) in lines {
        let text = std::fs::read_to_string(format!("{LIBC}{file}")).expect("libc source");
        assert!(text.contains(line), "the libc's {file} no longer says `{line}`");
        assert!(ours.contains(line), "the shim does not say `{line}`");
    }
}

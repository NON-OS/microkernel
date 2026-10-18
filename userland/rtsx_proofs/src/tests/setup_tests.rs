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

//! Taking the reader: claim, register map, the PCI command, and the two
//! DMA buffers, which stay mapped for as long as the driver holds them.

use nonos_libc::{events, live_grants, violations};

use super::rig::taken;

#[test]
fn the_dma_buffers_stay_mapped_while_the_driver_holds_them() {
    let drv = taken();
    let ev = events();
    assert!(ev.iter().all(|e| !e.starts_with("dma_unmap")), "{ev:?}");
    assert_eq!(live_grants(), 2, "{ev:?}");
    drv.resv.write_u32(0, 0xA5A5_A5A5);
    assert_eq!(drv.resv.read_u8(0), 0xA5);
    assert_eq!(violations(), 0);
}

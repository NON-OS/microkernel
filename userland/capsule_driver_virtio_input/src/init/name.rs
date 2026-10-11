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

//! The device's name, for the boot line: QEMU calls its functions "QEMU
//! Virtio Tablet", "QEMU Virtio Keyboard" and "QEMU Virtio Mouse".

use nonos_virtio::Mmio;

use crate::constants::cfg::{CFG_ID_NAME, NAME_MAX, SELECT, SIZE, SUBSEL, UNION};

/// The name's printable bytes and how many there are; anything else is cut.
pub fn name(cfg: Mmio) -> ([u8; NAME_MAX], usize) {
    let mut out = [0u8; NAME_MAX];
    cfg.w8(SELECT, CFG_ID_NAME);
    cfg.w8(SUBSEL, 0);
    let size = (cfg.r8(SIZE) as usize).min(NAME_MAX);
    let mut n = 0;
    for i in 0..size {
        let b = cfg.r8(UNION + i);
        if !(0x20..=0x7E).contains(&b) {
            break;
        }
        out[n] = b;
        n += 1;
    }
    (out, n)
}

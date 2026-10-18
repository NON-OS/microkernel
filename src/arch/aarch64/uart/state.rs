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

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

use crate::arch::fdt::find::uart::{UartInfo, UartKind};

/// QEMU virt's PL011, the console until the device tree names another.
static BASE: AtomicU64 = AtomicU64::new(0x0900_0000);
/// Kind, register shift and access width, packed one byte each so a reader
/// never sees one port's shift with another's kind.
static LAYOUT: AtomicU32 = AtomicU32::new(pack(UartKind::Pl011, 0, 4));
/// Set when a byte timed out: the next ones are dropped at once instead of
/// each costing a full wait, until the FIFO is seen to have room again.
static STALLED: AtomicBool = AtomicBool::new(false);

const fn pack(kind: UartKind, shift: u8, width: u8) -> u32 {
    let kind = match kind {
        UartKind::Pl011 => 0,
        UartKind::Ns16550 => 1,
        UartKind::MiniUart => 2,
    };
    kind | (shift as u32) << 8 | (width as u32) << 16
}

pub(super) fn select(info: &UartInfo) {
    LAYOUT.store(pack(info.kind, info.reg_shift, info.io_width), Ordering::Release);
    BASE.store(info.base, Ordering::Release);
    STALLED.store(false, Ordering::Relaxed);
}

pub fn uart_base() -> u64 {
    BASE.load(Ordering::Acquire)
}

pub(super) fn layout() -> (UartKind, u8, u8) {
    let packed = LAYOUT.load(Ordering::Acquire);
    let kind = match packed & 0xFF {
        0 => UartKind::Pl011,
        1 => UartKind::Ns16550,
        _ => UartKind::MiniUart,
    };
    (kind, (packed >> 8) as u8, (packed >> 16) as u8)
}

pub(super) fn stalled() -> bool {
    STALLED.load(Ordering::Relaxed)
}

pub(super) fn set_stalled(stalled: bool) {
    STALLED.store(stalled, Ordering::Relaxed);
}

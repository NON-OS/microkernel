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

//! The mailbox registers (raspberrypi/firmware wiki, "Mailboxes"). Mailbox 0
//! carries the VideoCore's messages to the ARM and mailbox 1 the ARM's to
//! the VideoCore; the ARM reads only the first and writes only the second.
//! Each has its data register at +0x00 and status at +0x18 of its own
//! block, +0x00 and +0x20 from the base. Status bit 31 is full, bit 30
//! empty. A message is a 16-byte aligned bus address with the channel in
//! its low four bits.

use crate::arch::aarch64::mmu::pointer::mapped;

const READ: u64 = 0x00;
const READ_STATUS: u64 = 0x18;
const WRITE: u64 = 0x20;
const WRITE_STATUS: u64 = 0x38;
const FULL: u32 = 1 << 31;
const EMPTY: u32 = 1 << 30;
/// Polls before a mailbox that never moves is given up on.
const SPINS: u32 = 50_000_000;

fn read(regs: u64, offset: u64) -> u32 {
    // SAFETY: `offset` names a mailbox register in the window `state` mapped
    // Device at `regs` for the kernel's lifetime.
    unsafe { core::ptr::read_volatile(mapped::<u32>(regs, offset)) }
}

fn wait(mut ready: impl FnMut() -> bool) -> bool {
    (0..SPINS).any(|_| {
        core::hint::spin_loop();
        ready()
    })
}

/// Hand the VideoCore `message`, once mailbox 1 has room for it.
pub(super) fn send(regs: u64, message: u32) -> bool {
    if !wait(|| read(regs, WRITE_STATUS) & FULL == 0) {
        return false;
    }
    // SAFETY: as for `read`.
    unsafe { core::ptr::write_volatile(mapped::<u32>(regs, WRITE), message) };
    true
}

/// The next message the VideoCore puts in mailbox 0 for `channel`; one for
/// another channel is not this caller's and is dropped.
pub(super) fn receive(regs: u64, channel: u32) -> Option<u32> {
    let mut found = None;
    let arrived = wait(|| {
        if read(regs, READ_STATUS) & EMPTY != 0 {
            return false;
        }
        let message = read(regs, READ);
        found = Some(message);
        message & 0xf == channel
    });
    arrived.then_some(found).flatten()
}

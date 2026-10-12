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

//! One property message to the firmware and its answer.
//!
//! The VideoCore reads and writes the buffer in memory, not through this
//! CPU's caches: it is written out before the message is sent and dropped
//! from the caches before the answer is read, and nothing touches it in
//! between. It lives in the kernel image, so it is used only when the
//! mailbox's bus window covers it, as it does wherever a Pi loads the image.

use spin::Mutex;

use super::message::{encode, Tag, WORDS};
use super::regs::{receive, send};
use super::state::mailbox;
use crate::arch::cache_flush::flush_and_fence;

/// The property channel, ARM to VideoCore.
const PROPERTY: u32 = 8;

#[repr(C, align(16))]
struct Buffer([u32; WORDS]);

static BUFFER: Mutex<Buffer> = Mutex::new(Buffer([0; WORDS]));

/// Ask `tags`; returns the answered buffer, for `message::answer` to read.
/// `None` when there is no mailbox, the buffer is out of its reach, or
/// nothing came back.
pub(super) fn call(tags: &[Tag]) -> Option<[u32; WORDS]> {
    let (info, regs) = mailbox()?;
    let mut buffer = BUFFER.lock();
    let bytes = encode(tags, &mut buffer.0).ok()? as u64;
    let va = buffer.0.as_ptr() as u64;
    let pa = crate::arch::aarch64::mmu::virt_to_phys(va)?;
    let bus = info.dma.bus_address(pa, bytes)?;
    let message = u32::try_from(bus).ok().filter(|b| b & 0xf == 0)? | PROPERTY;
    flush_and_fence(va, bytes);
    let sent = send(regs, message) && receive(regs, PROPERTY) == Some(message);
    flush_and_fence(va, bytes);
    if !sent {
        return None;
    }
    Some(buffer.0)
}

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

//! What the owner reads on serial: one `[ACPI] region` line per window the
//! capsule was granted or refused, and after each the posture line with the
//! counts, which the boot matrix reads from the last one in a log.

use core::sync::atomic::{AtomicU64, Ordering};

use super::kernel::records::Space;
use super::rule::Refusal;
use crate::sys::serial::Line;

static GRANTED: AtomicU64 = AtomicU64::new(0);
static REFUSED: AtomicU64 = AtomicU64::new(0);
/// Refusals past this are counted and not printed: firmware retrying a
/// refused access in a loop would otherwise bury the log.
const REFUSALS_PRINTED: u64 = 64;

pub(super) fn granted(space: Space, base: u64, len: u64) {
    GRANTED.fetch_add(1, Ordering::Relaxed);
    region(space, base, len).str(b" granted").end();
    posture();
}

pub(super) fn refused(space: Space, base: u64, len: u64, why: Refusal) {
    if REFUSED.fetch_add(1, Ordering::Relaxed) < REFUSALS_PRINTED {
        region(space, base, len).str(b" refused: ").str(why.reason().as_bytes()).end();
    }
    posture();
}

fn region(space: Space, base: u64, len: u64) -> Line {
    let mut line = Line::new();
    let name: &[u8] = match space {
        Space::Memory => b"[ACPI] region memory base=",
        Space::Io => b"[ACPI] region io base=",
        Space::Pci => b"[ACPI] region pci seg.bus.dev.fn=",
    };
    line.str(name).hex(base).str(b" len=").hex(len);
    line
}

fn posture() {
    Line::new()
        .str(b"[ACPI] posture regions granted=")
        .dec(GRANTED.load(Ordering::Relaxed))
        .str(b" refused=")
        .dec(REFUSED.load(Ordering::Relaxed))
        .end();
}

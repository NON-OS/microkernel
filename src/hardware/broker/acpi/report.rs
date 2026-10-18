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
//! capsule was granted or refused (each refusal once, `refusal_told`), then
//! the posture line, which the boot matrix reads from the last in a log.

use core::sync::atomic::{AtomicU64, Ordering};
use super::kernel::records::Space;
use super::rule::{Refusal, Tell};
use crate::sys::serial::Line;

#[path = "refusal_told.rs"]
mod told;

static GRANTED: AtomicU64 = AtomicU64::new(0);
static REFUSED: AtomicU64 = AtomicU64::new(0);

pub(super) fn granted(space: Space, base: u64, len: u64) {
    GRANTED.fetch_add(1, Ordering::Relaxed);
    region(space, base, len).str(b" granted").end();
    posture();
}

pub(super) fn refused(space: Space, base: u64, len: u64, why: Refusal) {
    REFUSED.fetch_add(1, Ordering::Relaxed);
    let verdict = told::told(space as u8, base, len);
    if verdict != Tell::Quiet {
        told::say(verdict, region(space, base, len), why.reason().as_bytes());
    }
    posture();
}

fn region(space: Space, base: u64, len: u64) -> Line {
    let name: &[u8] = match space {
        Space::Memory => b"[ACPI] region memory base=",
        Space::Io => b"[ACPI] region io base=",
        Space::Pci => b"[ACPI] region pci seg.bus.dev.fn=",
    };
    let mut line = Line::new();
    line.str(name).hex(base).str(b" len=").hex(len);
    line
}

/// Hardware errors follow the region counts, where old readers look.
pub(super) fn posture() {
    let (recoverable, fatal, corrected) = super::hw_errors::counts();
    Line::new()
        .str(b"[ACPI] posture regions granted=")
        .dec(GRANTED.load(Ordering::Relaxed))
        .str(b" refused=")
        .dec(REFUSED.load(Ordering::Relaxed))
        .str(b" hw_errors fatal=")
        .dec(fatal)
        .str(b" recoverable=")
        .dec(recoverable)
        .str(b" corrected=")
        .dec(corrected)
        .str(super::ecam_note::suffix())
        .str(super::sleep_adopt::soft_off_posture())
        .end();
}

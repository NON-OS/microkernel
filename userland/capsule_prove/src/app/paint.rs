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

//! Each step as a line, marked done, refused or under way, and what a key
//! does now. The floor is stated whether or not the machine meets it.

use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;

use super::manifest::{HEIGHT, WIDTH};
use super::memory::{Memory, HEAP_MIB, SPARE_MIB};
use super::state::{Line, Mark, Stage};

const BG: u32 = 0xFF10161C;
const ACCENT: u32 = 0xFF00D4AA;
const TEXT: u32 = 0xFFE8F0F8;
const DIM: u32 = 0xFF667788;
const REFUSED: u32 = 0xFFE05A5A;

pub fn paint(fb: &mut PaintBuffer, log: &[Line], stage: Stage) {
    fb.clear(BG);
    fb.fill_rect(0, 0, WIDTH, 4, ACCENT);
    fb.text_scaled(24, 22, b"Prove this device", ACCENT, 2);
    fb.text(24, 58, b"An enrolled bootloader, kernel and device, without naming it", DIM);
    for (i, line) in log.iter().enumerate() {
        let y = 92 + 24 * i as u32;
        let (mark, colour): (&[u8], u32) = match line.mark {
            Mark::Ok => (b"ok", ACCENT),
            Mark::No => (b"no", REFUSED),
            Mark::Note => (b"..", DIM),
        };
        fb.text(24, y, mark, colour);
        fb.text(56, y, line.text.as_bytes(), TEXT);
    }
    let hint: &[u8] = match stage {
        Stage::Ready => b"Enter proves to this verifier. Esc closes.",
        Stage::Running(_) => b"Working. Each step shows here as it ends.",
        Stage::Done => b"Saved. Enter proves again; Esc closes.",
        Stage::Refused => b"Nothing was proven. Enter starts again; Esc closes.",
    };
    fb.text(24, HEIGHT - 32, hint, DIM);
}

/// Whether the prover's heap is held, and the floor in words.
pub fn memory_line(m: Memory) -> (bool, String) {
    let floor = HEAP_MIB + SPARE_MIB;
    match m {
        Memory::Held => (true, alloc::format!("Memory: {HEAP_MIB} MiB held for the prover")),
        Memory::Short(free) => (
            false,
            alloc::format!("Memory: {free} MiB free; a proof needs {floor} MiB, for a 2.6 GB peak"),
        ),
        Memory::Unmapped => (false, alloc::format!("Memory: {HEAP_MIB} MiB free but not mappable")),
        Memory::Unknown => (false, "Memory: the kernel did not say how much is free".into()),
    }
}

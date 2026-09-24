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

//! Laying a program's segments into a guest's address space.
//!
//! The kernel maps pages on request and copies bytes on request; deciding
//! which pages and which bytes is the personality's job, because the
//! format is the personality's knowledge.

use super::elf::{Elf, PF_W, PF_X, PT_INTERP, PT_LOAD};
use super::phdr::Phdr;
use crate::linux::guest::Guest;

pub enum LoadError {
    NotElf,
    Dynamic,
    Map,
    Copy,
}

/// Map every `PT_LOAD` and return the entry point. A dynamic executable is
/// refused here rather than half loaded: its interpreter is the next phase
/// and a half loaded image would fault in a way nobody could read.
pub fn load(guest: &Guest, bytes: &[u8]) -> Result<u64, LoadError> {
    let elf = Elf::parse(bytes).ok_or(LoadError::NotElf)?;
    for i in 0..elf.phnum() {
        let Some(ph) = elf.phdr(i) else { continue };
        if ph.kind == PT_INTERP {
            return Err(LoadError::Dynamic);
        }
    }
    for i in 0..elf.phnum() {
        let Some(ph) = elf.phdr(i) else { continue };
        if ph.kind == PT_LOAD && ph.memsz > 0 {
            segment(guest, bytes, &ph)?;
        }
    }
    Ok(elf.entry)
}

/// One segment: pages first, then the file bytes into them. The gap
/// between `filesz` and `memsz` is left as the zeroes the fresh frames
/// already hold, which is what a `.bss` is.
fn segment(guest: &Guest, bytes: &[u8], ph: &Phdr) -> Result<(), LoadError> {
    let write = ph.flags & PF_W != 0;
    let exec = ph.flags & PF_X != 0;
    if guest.map(ph.vaddr, ph.memsz, write, exec) < 0 {
        return Err(LoadError::Map);
    }
    if ph.filesz == 0 {
        return Ok(());
    }
    let from = ph.offset as usize;
    let to = from + ph.filesz as usize;
    let body = bytes.get(from..to).ok_or(LoadError::NotElf)?;
    if guest.write(ph.vaddr, body) < 0 {
        return Err(LoadError::Copy);
    }
    Ok(())
}

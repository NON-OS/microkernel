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

//! Enough ELF to find the program headers of a 64-bit little-endian
//! x86_64 image. Parsing lives here and not in the kernel: a malformed
//! header is this capsule's problem and nobody else's.

use super::phdr::Phdr;
use super::read::{u16v, u32v, u64v};

pub const PT_LOAD: u32 = 1;
pub const PT_INTERP: u32 = 3;
pub const PF_X: u32 = 1;
pub const PF_W: u32 = 2;

pub struct Elf<'a> {
    pub bytes: &'a [u8],
    pub entry: u64,
    phoff: u64,
    phentsize: u16,
    phnum: u16,
}

impl<'a> Elf<'a> {
    /// Refuses anything that is not a 64-bit little-endian x86_64
    /// executable, because this personality can host nothing else.
    pub fn parse(bytes: &'a [u8]) -> Option<Elf<'a>> {
        if bytes.len() < 64 || &bytes[0..4] != b"\x7fELF" {
            return None;
        }
        if bytes[4] != 2 || bytes[5] != 1 || u16v(bytes, 18)? != 0x3E {
            return None;
        }
        Some(Elf {
            bytes,
            entry: u64v(bytes, 24)?,
            phoff: u64v(bytes, 32)?,
            phentsize: u16v(bytes, 54)?,
            phnum: u16v(bytes, 56)?,
        })
    }

    pub fn phdr(&self, index: u16) -> Option<Phdr> {
        let at = (self.phoff + index as u64 * self.phentsize as u64) as usize;
        Some(Phdr {
            kind: u32v(self.bytes, at)?,
            flags: u32v(self.bytes, at + 4)?,
            offset: u64v(self.bytes, at + 8)?,
            vaddr: u64v(self.bytes, at + 16)?,
            filesz: u64v(self.bytes, at + 32)?,
            memsz: u64v(self.bytes, at + 40)?,
        })
    }

    pub fn phnum(&self) -> u16 {
        self.phnum
    }
}

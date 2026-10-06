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

//! Minimal PE images laid out by the PE/COFF specification, PE32+ or PE32,
//! with a section table in any order and an optional certificate table.

pub(super) const LFANEW: usize = 0x40;
pub(super) const OPT: usize = LFANEW + 24;
pub(super) const HEADERS: usize = 0x200;

pub(super) fn put(f: &mut [u8], at: usize, v: &[u8]) {
    f[at..at + v.len()].copy_from_slice(v);
}

/// Where the checksum and the certificate directory entry sit for a layout.
pub(super) fn fields(plus: bool) -> (usize, usize) {
    (OPT + 64, OPT + if plus { 112 } else { 96 } + 32)
}

/// Sections as `(file offset, size, fill byte)`, in table order.
pub(super) fn image(plus: bool, sections: &[(usize, usize, u8)]) -> Vec<u8> {
    let end = sections.iter().map(|s| s.0 + s.1).max().unwrap_or(HEADERS).max(HEADERS);
    let mut f = vec![0u8; end];
    let opt_size: usize = if plus { 240 } else { 224 };
    put(&mut f, 0, b"MZ");
    put(&mut f, 0x3C, &(LFANEW as u32).to_le_bytes());
    put(&mut f, LFANEW, b"PE\0\0");
    put(&mut f, LFANEW + 4, &0x8664u16.to_le_bytes());
    put(&mut f, LFANEW + 6, &(sections.len() as u16).to_le_bytes());
    put(&mut f, LFANEW + 20, &(opt_size as u16).to_le_bytes());
    put(&mut f, OPT, &(if plus { 0x20Bu16 } else { 0x10B }).to_le_bytes());
    put(&mut f, OPT + 60, &(HEADERS as u32).to_le_bytes());
    put(&mut f, OPT + 64, &0xC0FFEEu32.to_le_bytes());
    put(&mut f, OPT + if plus { 108 } else { 92 }, &16u32.to_le_bytes());
    for (i, &(at, size, fill)) in sections.iter().enumerate() {
        let s = OPT + opt_size + 40 * i;
        put(&mut f, s, b".sect\0\0\0");
        put(&mut f, s + 16, &(size as u32).to_le_bytes());
        put(&mut f, s + 20, &(at as u32).to_le_bytes());
        f[at..at + size].fill(fill);
    }
    f
}

/// Append a certificate table and point the directory entry at it, as a
/// signing tool does: 8-byte aligned, at the end of the file.
pub(super) fn sign(f: &mut Vec<u8>, plus: bool, cert: &[u8]) {
    while !f.len().is_multiple_of(8) {
        f.push(0);
    }
    let at = f.len();
    f.extend_from_slice(cert);
    let (_, entry) = fields(plus);
    put(f, entry, &(at as u32).to_le_bytes());
    put(f, entry + 4, &(cert.len() as u32).to_le_bytes());
}

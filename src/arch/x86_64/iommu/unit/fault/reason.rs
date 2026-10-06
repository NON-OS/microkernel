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

//! What a fault reason code means, in the words of VT-d 3.4 Appendix A and
//! of Linux intel/dmar.c (dma_remap_fault_reasons, irq_remap_fault_reasons),
//! so a log photo names the cause without the spec at hand.

/// Codes 0x20 to 0x2F come from interrupt requests; the record's address
/// field then holds the interrupt index in bits 63:48, not a page.
pub const fn is_interrupt(reason: u8) -> bool {
    reason >= 0x20 && reason < 0x30
}

pub const fn reason_text(reason: u8) -> &'static [u8] {
    match reason {
        0x01 => b"root entry not present",
        0x02 => b"context entry not present",
        0x03 => b"invalid context entry",
        0x04 => b"address beyond MGAW",
        0x05 => b"page not writable",
        0x06 => b"page not readable",
        0x07 => b"next table pointer invalid",
        0x08 => b"root table address invalid",
        0x09 => b"context table pointer invalid",
        0x0A => b"reserved bits set in root entry",
        0x0B => b"reserved bits set in context entry",
        0x0C => b"reserved bits set in page entry",
        0x0D => b"translation blocked by context entry",
        0x20 => b"reserved bits set in remapped interrupt",
        0x21 => b"interrupt index beyond table size",
        0x22 => b"interrupt entry not present",
        0x23 => b"interrupt table address invalid",
        0x24 => b"reserved bits set in interrupt entry",
        0x25 => b"compatibility format interrupt blocked",
        0x26 => b"interrupt source id check failed",
        _ => b"reason not listed",
    }
}

/// `bb:dd.f` for a requester id, as lspci prints it, so the faulting device
/// can be looked up on the same machine under Linux.
pub fn bdf_text(source: u16) -> [u8; 7] {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let bus = (source >> 8) as u8;
    let device = ((source >> 3) & 0x1F) as u8;
    let function = (source & 0x7) as u8;
    [
        HEX[(bus >> 4) as usize],
        HEX[(bus & 0xF) as usize],
        b':',
        HEX[(device >> 4) as usize],
        HEX[(device & 0xF) as usize],
        b'.',
        HEX[function as usize],
    ]
}

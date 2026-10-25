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

//! A development image's own attestation document on the serial line, so
//! its boot carries the evidence a verifier checks: the document the kernel
//! encodes, in hex, between a begin and an end line. Only under
//! `nonos-dev-attest`, which the build admits only with the development
//! loader, so no image the seal would release prints it. Written to the
//! UART alone, never into the log tail: `MkLogTail` answers capsules that
//! hold Network, and a quote must never reach one (`can_attest_doc`).

/// Fixed, since the evidence answers no live session: 32 bytes a verifier
/// checks the document's challenge against.
#[cfg(feature = "nonos-dev-attest")]
pub(crate) const CHALLENGE: [u8; 32] = *b"nonos dev attest: boot evidence.";

/// Once every capsule is started, so the registry the quote binds is the
/// booted one.
pub(super) fn dump() {
    #[cfg(feature = "nonos-dev-attest")]
    match crate::security::attest_doc::attest(&CHALLENGE) {
        Ok(doc) => print(&doc.encode()),
        Err(e) => uart(alloc::format!("[ATTEST-DOC] refused: {}", e.as_str()).as_bytes()),
    }
}

#[cfg(feature = "nonos-dev-attest")]
fn print(doc: &[u8]) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    uart(alloc::format!("[ATTEST-DOC] begin {}", doc.len()).as_bytes());
    let mut line = [0u8; 13 + 2 * 64];
    line[..13].copy_from_slice(b"[ATTEST-DOC] ");
    for chunk in doc.chunks(64) {
        for (i, b) in chunk.iter().enumerate() {
            line[13 + 2 * i] = HEX[usize::from(b >> 4)];
            line[14 + 2 * i] = HEX[usize::from(b & 15)];
        }
        uart(&line[..13 + 2 * chunk.len()]);
    }
    uart(b"[ATTEST-DOC] end");
}

/// One line to the UART, past the tail every other console line feeds.
#[cfg(feature = "nonos-dev-attest")]
fn uart(line: &[u8]) {
    crate::sys::serial::core::with_serial_lock(|| {
        line.iter().chain(b"\r\n").for_each(|b| crate::arch::console::write_byte(*b));
    });
}

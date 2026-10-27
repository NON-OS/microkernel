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

//! One device's line of the hardware report: its PCI ids and class, then
//! what NONOS does with it. Pure, so the proofs read the very text a boot
//! prints. Nothing on it names one machine: no serial, MAC or address.

/// The longest line `text` writes.
pub const TEXT_MAX: usize = 120;

/// What NONOS does with one device, for its line.
pub enum Verdict<'a> {
    /// The driver capsule that takes it.
    Driver(&'a str),
    /// Known, and no driver takes it, for this reason.
    Held(&'a str),
    /// No table names it.
    Unknown,
}

/// `[HW] vvvv:dddd cc/ss ` and the verdict, into `out`; the length written.
pub fn text(out: &mut [u8; TEXT_MAX], ids: (u16, u16), class: (u8, u8), v: &Verdict) -> usize {
    let mut n = 0;
    let mut put = |bytes: &[u8]| {
        let take = bytes.len().min(TEXT_MAX - n);
        out[n..n + take].copy_from_slice(&bytes[..take]);
        n += take;
    };
    put(b"[HW] ");
    put(&hex(ids.0 as u32, 4));
    put(b":");
    put(&hex(ids.1 as u32, 4));
    put(b" ");
    put(&hex(class.0 as u32, 2)[2..]);
    put(b"/");
    put(&hex(class.1 as u32, 2)[2..]);
    match v {
        Verdict::Driver(d) => {
            put(b" driven by ");
            put(d.as_bytes());
        }
        Verdict::Held(why) => {
            put(b" known, held: ");
            put(why.as_bytes());
        }
        Verdict::Unknown => put(b" unknown"),
    }
    n
}

/// The low `digits` hex digits of `v`, right-aligned in four bytes.
fn hex(v: u32, digits: u32) -> [u8; 4] {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = [b'0'; 4];
    for i in 0..digits as usize {
        out[3 - i] = HEX[((v >> (4 * i)) & 0xF) as usize];
    }
    out
}

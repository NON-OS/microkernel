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

//! The boot line for a remapping unit whose scope list was cut short. Its
//! devices past the cut are not translated, so their claims are refused
//! (fail closed, K11); the line says so, by a count and never a requester id
//!. Pure and allocation free, written into a fixed buffer because DMAR
//! is read before the heap is. kernel_proofs::dmar_scope truncated_tests.

pub(crate) const LINE_MAX: usize = 128;

/// The line for unit `unit` (its index in DMAR order), which named `named`
/// PCI device scopes of which the kernel keeps `keeps`, into `out`; its length.
pub(crate) fn line(unit: usize, named: u16, keeps: usize, out: &mut [u8; LINE_MAX]) -> usize {
    let mut at = 0;
    let mut put = |t: &[u8]| {
        let n = t.len().min(LINE_MAX - at);
        out[at..at + n].copy_from_slice(&t[..n]);
        at += n;
    };
    put(b"[DMAR] remapping unit ");
    let mut d = [0u8; 20];
    put(decimal(unit as u64, &mut d));
    if usize::from(named) > keeps {
        put(b" names ");
        put(decimal(u64::from(named), &mut d));
        put(b" device scopes; NONOS keeps ");
        put(decimal(keeps as u64, &mut d));
        put(b"; devices past that are refused");
    } else {
        put(b": a device scope could not be read; the devices it names are refused");
    }
    at
}

/// `v` in decimal, written at the end of `d`; the digits.
fn decimal(mut v: u64, d: &mut [u8; 20]) -> &[u8] {
    let mut at = d.len();
    loop {
        at -= 1;
        d[at] = b'0' + (v % 10) as u8;
        v /= 10;
        if v == 0 {
            return &d[at..];
        }
    }
}

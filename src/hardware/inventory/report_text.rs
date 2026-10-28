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

//! One device's line of the hardware report: where it sits and its ids,
//! then what NONOS does with it. Pure, so the proofs read the very text a
//! boot prints. Nothing on it names one machine: no serial, MAC or address.

use super::report_hex::{hex, upper};
pub(super) use super::report_kinds::{Ids, Verdict, TEXT_MAX};

/// `[HW] ` and the ids, then the verdict, into `out`; the length written.
pub(super) fn text(out: &mut [u8; TEXT_MAX], ids: &Ids, v: &Verdict) -> usize {
    let mut n = 0;
    let mut put = |bytes: &[u8]| {
        let take = bytes.len().min(TEXT_MAX - n);
        out[n..n + take].copy_from_slice(&bytes[..take]);
        n += take;
    };
    put(b"[HW] ");
    match ids {
        Ids::Pci((v, d), (c, s)) => {
            for part in [&hex(*v as u32)[..], b":", &hex(*d as u32), b" ", &hex(*c as u32)[2..]] {
                put(part);
            }
            put(b"/");
            put(&hex(*s as u32)[2..]);
        }
        Ids::Pnp(d) => {
            put(b"acpi PNP");
            put(&upper(hex(*d as u32)));
        }
        Ids::Other(bus, (v, d)) => {
            for part in [*bus, b" ", &hex(*v as u32), b":", &hex(*d as u32)] {
                put(part);
            }
        }
    }
    match v {
        Verdict::Driver(d) => [b" driven by ".as_slice(), d.as_bytes()].iter().for_each(|p| put(p)),
        Verdict::Held(why) => {
            [b" known, held: ".as_slice(), why.as_bytes()].iter().for_each(|p| put(p))
        }
        Verdict::NotNeeded => put(b" needs no driver"),
        Verdict::Unknown => put(b" unknown"),
    }
    n
}

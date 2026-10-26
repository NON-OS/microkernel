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
//! The verifier's trust file, `nonos-receipt-trust 1`: a header, then lines
//! `release <version> <pcr8> <pcr9>`, `supervisor <meas> <caps>` and
//! one `ak <key>`, hex throughout, `#` comments. One file has one meaning;
//! a line this reader does not know is refused.

use alloc::string::ToString;
use alloc::vec::Vec;

use crate::trust::{Release, Supervisor, Trust};

pub fn read(text: &str) -> Result<Trust, (usize, &'static str)> {
    let (mut releases, mut supervisors, mut ak) = (Vec::new(), Vec::new(), None);
    let mut header = false;
    for (i, raw) in text.lines().enumerate() {
        let w: Vec<&str> = raw.split_whitespace().collect();
        let at = |why| (i + 1, why);
        match (header, w.as_slice()) {
            (_, []) => {}
            (_, [c, ..]) if c.starts_with('#') => {}
            (false, ["nonos-receipt-trust", "1"]) => header = true,
            (false, _) => return Err(at("the first line is `nonos-receipt-trust 1`")),
            (true, ["release", v, p8, p9]) => releases.push(Release {
                version: v.to_string(),
                pcr8: hex(p8).ok_or(at("a PCR is 64 hex digits"))?,
                pcr9: hex(p9).ok_or(at("a PCR is 64 hex digits"))?,
            }),
            (true, ["supervisor", m, c]) => supervisors.push(Supervisor {
                meas: hex(m).ok_or(at("a measurement is 64 hex digits"))?,
                caps: u64::from_str_radix(c.trim_start_matches("0x"), 16)
                    .map_err(|_| at("caps are a hex mask"))?,
            }),
            (true, ["ak", k]) if ak.is_none() => {
                ak = Some(hex(k).ok_or(at("the AK is 128 hex digits"))?)
            }
            (true, ["ak", _]) => return Err(at("the AK is named once")),
            (true, _) => return Err(at("a line is release, supervisor, ak or a # comment")),
        }
    }
    match (header, ak, releases.is_empty() || supervisors.is_empty()) {
        (false, ..) => Err((0, "the first line is `nonos-receipt-trust 1`")),
        (_, None, _) => Err((0, "no ak line")),
        (_, _, true) => Err((0, "at least one release and one supervisor")),
        (_, Some(ak), false) => Ok(Trust { releases, supervisors, ak }),
    }
}

pub fn hex<const N: usize>(s: &str) -> Option<[u8; N]> {
    let b = s.as_bytes();
    if b.len() != 2 * N {
        return None;
    }
    let digit = |c: u8| (c as char).to_digit(16).map(|d| d as u8);
    let mut out = [0u8; N];
    for (i, o) in out.iter_mut().enumerate() {
        *o = digit(b[2 * i])? << 4 | digit(b[2 * i + 1])?;
    }
    Some(out)
}

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


//! The descriptor's "pow-params" line, in the inner encrypted layer:
//! `pow-params v1 <seed, base64> <suggested effort> <expiry, ISO time>`.

use crate::directory::base64;
use crate::directory::lines::{arg, lines};
use crate::directory::number::decimal;
use crate::directory::time;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PowParams {
    pub seed: [u8; 32],
    pub suggested_effort: u32,
    /// When the seed stops being current, in seconds since the epoch. A
    /// cached descriptor whose seed has expired is fetched again.
    pub expires: u64,
}

/// A v1 line that does not parse, or a second puzzle line. The fork refuses
/// the descriptor for either.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BadPowParams;

/// The puzzle a descriptor's inner layer asks for. `Ok(None)` when it asks
/// for none, or only for a kind this client does not know, which it may
/// ignore since solving is never required. `Err` when a v1 line is
/// malformed or a second line appears: the fork refuses that descriptor,
/// and so does this client.
///
/// Only the header before the first introduction point counts; a line of
/// that name inside an introduction point's section is not the service's.
pub fn params(inner: &[u8]) -> Result<Option<PowParams>, BadPowParams> {
    let mut found = None;
    let mut seen = false;
    for line in lines(inner) {
        if line.keyword == b"introduction-point" {
            break;
        }
        if line.keyword != b"pow-params" {
            continue;
        }
        if seen {
            return Err(BadPowParams);
        }
        seen = true;
        if arg(line.rest, 0) != Some(b"v1".as_slice()) {
            continue;
        }
        found = Some(v1(line.rest).ok_or(BadPowParams)?);
    }
    Ok(found)
}

fn v1(rest: &[u8]) -> Option<PowParams> {
    let seed: [u8; 32] = base64::decode(arg(rest, 1)?)?.try_into().ok()?;
    let effort = decimal(arg(rest, 2)?)?;
    let suggested_effort = u32::try_from(effort).ok()?;
    let expires = iso_time(arg(rest, 3)?)?;
    Some(PowParams { seed, suggested_effort, expires })
}

/// `YYYY-MM-DDTHH:MM:SS`, the form parse_iso_time_nospace reads. The
/// directory parser takes the same fields with a space in place of the T.
fn iso_time(text: &[u8]) -> Option<u64> {
    if text.len() != 19 || text[10] != b'T' {
        return None;
    }
    let mut spaced = [0u8; 19];
    spaced.copy_from_slice(text);
    spaced[10] = b' ';
    time::parse(&spaced)
}

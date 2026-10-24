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

//! SET_VCP, and the bodies other parts of the compositor already read:
//! accessibility settings, a host's size hint, and a driver's notice.

use super::ops::{b, some, Op};
use crate::display::{access_wire, hint_rule::Hint};
use crate::requests::vcp;

/// Three outputs here; five filters; contrast and follow yes or no; the
/// seven magnifier steps.
fn access(w: &[u32]) -> Option<Vec<u64>> {
    let mag = [4, 6, 8, 12, 16, 24, 32].contains(&w[3]);
    (w[0] < 3 && w[1] <= 4 && w[2] <= 1 && mag && w[4] <= 1).then(|| some(&w[..5]))?
}

fn access_read(x: &[u8]) -> Option<Vec<u64>> {
    let (o, a) = access_wire::parse(x, 3)?;
    some(&[o as u32, a.filter, b(a.contrast), a.magnify, a.follow])
}

fn hint(x: &[u8]) -> Option<Vec<u64>> {
    Hint::decode(x).map(|h| some(&[h.output, h.width, h.height, h.source]))?
}

pub fn set_vcp(x: &[u8]) -> Option<Vec<u64>> {
    vcp::set_vcp(x).map(|s| some(&[s.panel, s.code.into(), s.value.into()]))?
}

/// A code that fits a byte and a value that fits 16 bits.
pub fn vcp_model(w: &[u32]) -> Option<Vec<u64>> {
    (w[1] < 256 && w[2] < 65536).then(|| some(&w[..3]))?
}

fn why(x: &[u8]) -> Option<Vec<u64>> {
    some(&[nonos_gfx_wire::notice::why(x)?])
}

pub fn others() -> Vec<Op> {
    vec![
        ("accessibility", 20, &[1, 2, 1, 8, 1], access_read, access),
        // Source 1 is VMMDev, 2 VMware.
        ("display_hint", 16, &[0, 1024, 768, 1], hint, |w| {
            matches!(w[3], 1 | 2).then(|| some(&w[..4]))?
        }),
        // 1: outputs changed; 2: the device is leaving.
        ("output_notice", 4, &[2], why, |w| matches!(w[0], 1 | 2).then(|| some(&w[..1]))?),
    ]
}

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

//! The Country element (IEEE Std 802.11-2020, 9.4.2.8): the country an access
//! point says it is in and the channels it says are open there. Channel
//! triplets name a first channel and a count, stepping by one on 2.4 GHz and
//! by four on 5 GHz; a triplet whose first octet is 201 or more is an
//! operating extension and names no channels.

use super::channel_set::ChannelSet;

pub const EID_COUNTRY: u8 = 7;
const FIRST_OPERATING_EXTENSION: u8 = 201;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Country {
    pub alpha2: [u8; 2],
    /// ' ' any environment, 'O' outdoor, 'I' indoor, 'X' non-country.
    pub environment: u8,
    pub channels: ChannelSet,
}

/// Parse a Country element's body (after the ID and length octets). `None`
/// when it is shorter than its country string or lists no channel at all:
/// an element that names nothing restricts nothing.
pub fn parse_country(body: &[u8]) -> Option<Country> {
    let (head, triplets) = (body.get(..3)?, &body[3..]);
    let mut channels = ChannelSet::EMPTY;
    for t in triplets.chunks_exact(3) {
        let (first, count) = (t[0], t[1]);
        if first == 0 || first >= FIRST_OPERATING_EXTENSION {
            continue;
        }
        let step = if first <= 14 { 1 } else { 4 };
        for i in 0..count {
            match (i as u32).checked_mul(step).and_then(|o| u8::try_from(first as u32 + o).ok()) {
                Some(ch) => channels.insert(ch),
                None => break,
            }
        }
    }
    if channels.is_empty() {
        return None;
    }
    Some(Country { alpha2: [head[0], head[1]], environment: head[2], channels })
}

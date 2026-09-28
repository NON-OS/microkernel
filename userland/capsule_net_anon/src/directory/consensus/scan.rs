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

//! Walking a consensus once, building its entries as the lines go by.

use crate::directory::lines::lines;
use crate::directory::time::parse as parse_time;

use super::build::{collect, push, set};
use super::document::Consensus;
use super::entry::Entry;
use super::span::signed_range;
use super::{flags, router, weights};

/// Parse a microdescriptor flavour consensus.
///
pub fn parse(body: &[u8]) -> Option<Consensus> {
    let signed = signed_range(body)?;
    let mut out = Consensus::empty(signed);
    let mut current: Option<Entry> = None;
    let mut flavoured = false;
    for line in lines(body) {
        match line.keyword {
            b"network-status-version" => flavoured = line.rest.ends_with(b"microdesc"),
            b"valid-after" => out.valid_after = parse_time(line.rest)?,
            b"fresh-until" => out.fresh_until = parse_time(line.rest)?,
            b"valid-until" => out.valid_until = parse_time(line.rest)?,
            b"bandwidth-weights" => out.weights = weights::parse(line.rest),
            b"r" => {
                push(&mut out.entries, current.take());
                current = router::parse(line.rest);
            }
            b"m" => set(&mut current, |e| {
                if let Some(digest) = router::microdesc_digest(line.rest) {
                    e.microdesc_digest = digest;
                }
            }),
            b"s" => set(&mut current, |e| e.flags = flags::parse(line.rest)),
            b"w" => set(&mut current, |e| e.weight = flags::bandwidth(line.rest)),
            b"directory-signature" => {
                push(&mut out.entries, current.take());
                collect(&mut out, body, line.at, line.rest);
            }
            _ => {}
        }
    }
    push(&mut out.entries, current.take());
    if !flavoured || out.valid_until <= out.valid_after {
        return None;
    }
    Some(out)
}

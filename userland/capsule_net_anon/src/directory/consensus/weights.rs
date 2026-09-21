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

//! The `bandwidth-weights` line, which says how to weigh each position.

use crate::directory::lines::args;
use crate::directory::number::decimal;
use crate::path::Weights;

/// Read the eight weights this capsule uses off a `bandwidth-weights` line.
///
pub fn parse(rest: &[u8]) -> Weights {
    let mut weights = Weights::default();
    for field in args(rest) {
        let Some((name, value)) = split(field) else { continue };
        let Some(value) = decimal(value) else { continue };
        if value > u32::MAX as u64 {
            continue;
        }
        let value = value as u32;
        match name {
            b"Wgg" => weights.wgg = value,
            b"Wgd" => weights.wgd = value,
            b"Wmg" => weights.wmg = value,
            b"Wmd" => weights.wmd = value,
            b"Wme" => weights.wme = value,
            b"Wmm" => weights.wmm = value,
            b"Wee" => weights.wee = value,
            b"Wed" => weights.wed = value,
            _ => {}
        }
    }
    weights
}

fn split(field: &[u8]) -> Option<(&[u8], &[u8])> {
    let at = field.iter().position(|b| *b == b'=')?;
    Some((&field[..at], field.get(at + 1..)?))
}

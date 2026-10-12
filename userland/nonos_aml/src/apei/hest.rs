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

//! HEST, the Hardware Error Source Table (ACPI 6.5, 18.3.2): each error
//! source the platform reports through. NONOS reads the Generic Hardware
//! Error Sources (GHES, type 9, and GHESv2, type 10, 18.3.2.7-8), whose
//! errors firmware writes to memory; the other types are stepped over by
//! their length, and the walk stops at a type it does not know.

use super::le::{u16_at, u32_at};
use super::source::{ghes, Ghes};
use alloc::vec::Vec;

const HEADER: usize = 40;
const MAX_SOURCES: usize = 256;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorSource {
    pub kind: u16,
    pub id: u16,
    pub enabled: bool,
    pub ghes: Option<Ghes>,
}

/// Each type's length; the machine-check types add 28 bytes per bank,
/// their bank count at the byte given.
fn length(t: &[u8], at: usize, kind: u16) -> Option<usize> {
    let banks = |off: usize| t.get(at + off).map(|n| *n as usize * 28);
    Some(match kind {
        0 => 40 + banks(32)?,
        1 | 11 => 48 + banks(44)?,
        2 => 20,
        6 => 48,
        7 => 44,
        8 => 56,
        9 => 64,
        10 => 92,
        _ => return None,
    })
}

pub fn hest(t: &[u8]) -> Vec<ErrorSource> {
    let mut out = Vec::new();
    let count = t.get(36..40).map_or(0, |_| u32_at(t, 36) as usize).min(MAX_SOURCES);
    let mut at = HEADER;
    for _ in 0..count {
        let Some(kind) = t.get(at..at + 2).map(|_| u16_at(t, at)) else { break };
        let Some(len) = length(t, at, kind) else { break };
        let Some(s) = t.get(at..at + len) else { break };
        let enabled = matches!(kind, 0 | 1 | 6..=11) && s[7] != 0;
        let g = matches!(kind, 9 | 10).then(|| ghes(s, kind == 10)).flatten();
        out.push(ErrorSource { kind, id: u16_at(s, 2), enabled, ghes: g });
        at += len;
    }
    out
}

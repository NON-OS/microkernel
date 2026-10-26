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
//! One DMAR read as the kernel reads it: admitted by its header, walked,
//! and each DRHD, RMRR and ANDD given to its parser; what came back is
//! held to each parser's rules here, the walk's own bounds in dmar_tests.

use super::admit::admitted;
use crate::dmar_scope::dmar_andd::{parse_andd, MAX_NAME};
use crate::dmar_scope::dmar_parse::parse_drhd;
use crate::dmar_scope::dmar_rmrr::{parse_rmrr, MAX_RMRR_SCOPES, RMRR_MAX};
use crate::dmar_scope::dmar_scope::{UnitScope, MAX_PATH};
use crate::dmar_scope::dmar_walk::walk_structures;

/// Where each structure handed on lay, whether the walk was cut short,
/// whether every parser kept its rules, and which kinds one kept (bit 0
/// DRHD, 1 RMRR, 2 ANDD).
#[derive(Default)]
pub struct Seen {
    pub at: Vec<(usize, usize)>,
    pub cut: bool,
    pub sound: bool,
    pub kept: u8,
}

/// A structure given to its kind's parser: `None` refused, else whether
/// what it kept holds that parser's rules.
fn parsed(kind: u16, s: &[u8]) -> Option<bool> {
    match kind {
        0 => {
            let mut u = Box::new(UnitScope::EMPTY);
            let paths = |u: &UnitScope| {
                u.scopes[..u.count as usize].iter().all(|c| c.path_len as usize <= MAX_PATH)
            };
            parse_drhd(s, &mut u).then(|| (u.count as usize) * 6 <= s.len() - 16 && paths(&u))
        }
        1 => parse_rmrr(s).map(|m| {
            let whole = m.base & 0xFFF == 0 && m.limit & 0xFFF == 0xFFF && m.limit >= m.base;
            whole && m.limit - m.base < RMRR_MAX && m.count as usize <= MAX_RMRR_SCOPES
        }),
        4 => parse_andd(s).map(|a| (1..=MAX_NAME).contains(&(a.name_len as usize))),
        _ => None,
    }
}

pub fn read(t: &[u8]) -> Option<Seen> {
    let t = admitted(t)?;
    let (base, mut seen) = (t.as_ptr() as usize, Seen { sound: true, ..Seen::default() });
    let walked = walk_structures(t, |kind, s| {
        let at = s.as_ptr() as usize - base;
        seen.at.push((at, at + s.len()));
        if let Some(sound) = parsed(kind, s) {
            (seen.sound, seen.kept) = (seen.sound && sound, seen.kept | 1 << kind.min(2));
        }
    });
    seen.cut = walked.cut_short;
    Some(seen)
}

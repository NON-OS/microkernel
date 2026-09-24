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

//! The gzip member header, and where its deflate stream starts.

/// Offset of the first deflate byte, or `None` when this is not a
/// member header.
pub(super) fn body_at(d: &[u8]) -> Option<usize> {
    if d.len() < 18 || d[0] != 0x1f || d[1] != 0x8b || d[2] != 8 {
        return None;
    }
    let mut p = 10usize;
    skip_extra(d, &mut p)?;
    skip_name(d, &mut p)?;
    skip_comment(d, &mut p)?;
    if d[3] & 2 != 0 {
        p = p.checked_add(2)?;
    }
    (p < d.len()).then_some(p)
}

fn skip_extra(d: &[u8], p: &mut usize) -> Option<()> {
    if d[3] & 4 == 0 {
        return Some(());
    }
    let xlen = *d.get(*p)? as usize | ((*d.get(*p + 1)? as usize) << 8);
    *p = p.checked_add(2)?.checked_add(xlen)?;
    (*p < d.len()).then_some(())
}

fn skip_name(d: &[u8], p: &mut usize) -> Option<()> {
    if d[3] & 8 == 0 {
        return Some(());
    }
    while *d.get(*p)? != 0 {
        *p = p.checked_add(1)?;
    }
    p.checked_add(1).map(|n| *p = n)
}

fn skip_comment(d: &[u8], p: &mut usize) -> Option<()> {
    if d[3] & 16 == 0 {
        return Some(());
    }
    while *d.get(*p)? != 0 {
        *p = p.checked_add(1)?;
    }
    p.checked_add(1).map(|n| *p = n)
}

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

//! Writing a tree in the PROPERTIES op's form (`wire`).

use super::wire::{Prop, Tree, Val, MAX, MAX_DEPTH, MAX_LEVELS, MAX_WIRE};
use alloc::vec::Vec;

fn name(out: &mut Vec<u8>, n: &[u8]) -> Option<()> {
    out.push(u8::try_from(n.len()).ok()?);
    out.extend_from_slice(n);
    Some(())
}

fn val(out: &mut Vec<u8>, v: &Val, depth: u32) -> Option<()> {
    match v {
        Val::Int(i) => {
            out.push(1);
            out.extend_from_slice(&i.to_le_bytes());
        }
        Val::Str(s) => {
            out.push(2);
            out.extend_from_slice(&u16::try_from(s.len()).ok()?.to_le_bytes());
            out.extend_from_slice(s);
        }
        Val::Ref(p) => {
            out.push(3);
            name(out, p)?;
        }
        Val::Pkg(e) if depth < MAX_DEPTH && e.len() <= MAX => {
            out.extend_from_slice(&[4, e.len() as u8]);
            e.iter().try_for_each(|x| val(out, x, depth + 1))?;
        }
        Val::Pkg(_) => return None,
    }
    (out.len() <= MAX_WIRE).then_some(())
}

fn prop(out: &mut Vec<u8>, p: &Prop) -> Option<()> {
    name(out, &p.name)?;
    val(out, &p.val, 0)
}

fn tree(out: &mut Vec<u8>, t: &Tree, level: u32) -> Option<()> {
    let fits = level < MAX_LEVELS && t.props.len() <= MAX && t.children.len() <= MAX;
    fits.then_some(())?;
    name(out, &t.name)?;
    out.push(t.props.len() as u8);
    t.props.iter().try_for_each(|p| prop(out, p))?;
    out.push(t.children.len() as u8);
    t.children.iter().try_for_each(|c| tree(out, c, level + 1))
}

/// None when the tree passes any of the wire's limits.
pub fn encode(t: &Tree) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    tree(&mut out, t, 0)?;
    (out.len() <= MAX_WIRE).then_some(out)
}

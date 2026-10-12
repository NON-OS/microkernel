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

//! Reading a tree in the PROPERTIES op's form (`wire`): every byte must be
//! accounted for, every count and depth within the wire's limits, and a
//! value of an unknown kind refuses the whole tree.

use super::wire::{Prop, Tree, Val, MAX_DEPTH, MAX_LEVELS, MAX_WIRE};
use super::wire_cursor::Cursor;

impl Cursor<'_> {
    fn val(&mut self, depth: u32) -> Option<Val> {
        Some(match self.byte()? {
            1 => Val::Int(u64::from_le_bytes(self.take(8)?.try_into().ok()?)),
            2 => {
                let n = u16::from_le_bytes(self.take(2)?.try_into().ok()?) as usize;
                Val::Str(self.take(n)?.to_vec())
            }
            3 => Val::Ref(self.name()?),
            4 if depth < MAX_DEPTH => {
                let n = self.count()?;
                Val::Pkg((0..n).map(|_| self.val(depth + 1)).collect::<Option<_>>()?)
            }
            _ => return None,
        })
    }

    fn tree(&mut self, level: u32) -> Option<Tree> {
        if level >= MAX_LEVELS {
            return None;
        }
        let name = self.name()?;
        let n = self.count()?;
        let prop = |r: &mut Self| Some(Prop { name: r.name()?, val: r.val(0)? });
        let props = (0..n).map(|_| prop(self)).collect::<Option<_>>()?;
        let n = self.count()?;
        let children = (0..n).map(|_| self.tree(level + 1)).collect::<Option<_>>()?;
        Some(Tree { name, props, children })
    }
}

pub fn decode(b: &[u8]) -> Option<Tree> {
    if b.len() > MAX_WIRE {
        return None;
    }
    let mut r = Cursor { b, at: 0 };
    let t = r.tree(0)?;
    (r.at == b.len()).then_some(t)
}

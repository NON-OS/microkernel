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

use alloc::rc::Rc;
use alloc::vec::Vec;

/* One custom property as element `owner` declared it; a None value is
 * invalid ('initial', a cycle, or an undefined reference). */
#[derive(Clone)]
pub(in crate::browser::css) struct Var {
    pub hash: u64,
    pub name: Rc<str>,
    pub value: Option<Rc<str>>,
    pub owner: usize,
}

/* Scopes stacked before a chain is flattened. */
const MAX_CHAIN: u8 = 8;

/* The custom properties an element sees: its own (sorted by hash) over
 * its parent's scope, which elements declaring none share. */
pub(in crate::browser::css) struct VarScope {
    parent: Option<Rc<VarScope>>,
    own: Vec<Var>,
    chain: u8,
}

impl VarScope {
    pub fn root() -> Rc<VarScope> {
        Rc::new(VarScope { parent: None, own: Vec::new(), chain: 0 })
    }

    /* A scope of `own` (sorted by hash) over `parent`. A deep chain is
     * flattened, nearest first so the stable sort and dedup keep the
     * nearest of equal names: a lookup never walks far. */
    pub fn child(parent: &Rc<VarScope>, own: Vec<Var>) -> Rc<VarScope> {
        if parent.chain < MAX_CHAIN {
            let (parent, chain) = (Some(parent.clone()), parent.chain + 1);
            return Rc::new(VarScope { parent, own, chain });
        }
        let (mut all, mut up) = (own, Some(parent.as_ref()));
        while let Some(s) = up {
            all.extend(s.own.iter().cloned());
            up = s.parent.as_deref();
        }
        all.sort_by_key(|v| v.hash);
        all.dedup_by(|b, a| a.hash == b.hash && a.name.eq_ignore_ascii_case(&b.name));
        Rc::new(VarScope { parent: None, own: all, chain: 0 })
    }

    /* The nearest declaration of `name`, whose hash is `h`. */
    pub fn find(&self, h: u64, name: &str) -> Option<&Var> {
        let scopes = core::iter::successors(Some(self), |s| s.parent.as_deref());
        let mut hashed = scopes.flat_map(|sc| {
            let at = sc.own.partition_point(|v| v.hash < h);
            sc.own[at..].iter().take_while(move |v| v.hash == h)
        });
        hashed.find(|v| v.name.eq_ignore_ascii_case(name))
    }
}

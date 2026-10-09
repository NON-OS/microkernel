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
use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::css::rule_index::key::hash_name;

use super::at::At;
use super::props::Props;
use super::scope::VarScope;
use super::subst::{substitute, Lookup, MAX_LEN};

/* Custom properties referring to each other followed before invalid. */
const MAX_DEPTH: u32 = 32;
pub(super) const PENDING: u8 = 0;
const BUSY: u8 = 1;
const DONE: u8 = 2;

/* The custom properties one element declares, (hash, name, raw value)
 * sorted by hash, resolved on demand: a property still being resolved
 * when it is asked for again is on a cycle, and has no value. */
pub(super) struct Own<'a> {
    pub raw: &'a [(u64, &'a str, &'a str)],
    pub state: Vec<u8>,
    pub vals: Vec<Option<Rc<str>>>,
    pub parent: &'a VarScope,
    pub props: &'a Props,
    pub id: usize,
    pub depth: u32,
}

impl Own<'_> {
    pub fn resolve(&mut self, i: usize) -> Option<Rc<str>> {
        if self.state[i] != PENDING || self.depth >= MAX_DEPTH {
            return self.vals[i].clone();
        }
        (self.state[i], self.depth) = (BUSY, self.depth + 1);
        let raw = self.raw[i].2;
        let v = substitute(raw, self).map(|v| String::from(v.trim()));
        let v = v.filter(|v| !v.eq_ignore_ascii_case("initial") && v.len() <= MAX_LEN);
        (self.state[i], self.depth, self.vals[i]) = (DONE, self.depth - 1, v.map(Rc::from));
        self.vals[i].clone()
    }
}

impl Lookup for Own<'_> {
    fn var(&mut self, name: &str) -> Option<Rc<str>> {
        let h = hash_name(name, true);
        let at = self.raw.partition_point(|r| r.0 < h);
        let hit = (at..self.raw.len())
            .take_while(|&k| self.raw[k].0 == h)
            .find(|&k| self.raw[k].1.eq_ignore_ascii_case(name));
        match hit {
            Some(k) => self.resolve(k),
            None => At { scope: self.parent, props: self.props, id: self.id }.var(name),
        }
    }
}

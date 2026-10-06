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

use crate::browser::css::rule_index::key::hash_name;

use super::{Counters, MAX_COUNTERS};

impl Counters {
    pub(super) fn reset(&mut self, h: u64, v: i32, scope: usize) {
        if let Some(c) = self.stack.iter_mut().rev().take_while(|c| c.2 == scope).find(|c| c.0 == h)
        {
            c.1 = v;
        } else if self.stack.len() < MAX_COUNTERS {
            self.stack.push((h, v, scope));
        }
    }

    pub(super) fn step(&mut self, h: u64, by: i32, scope: usize) {
        match self.stack.iter_mut().rev().find(|c| c.0 == h) {
            Some(c) => c.1 = c.1.saturating_add(by),
            None => self.reset(h, by, scope),
        }
    }

    pub(super) fn set(&mut self, h: u64, v: i32, scope: usize) {
        match self.stack.iter_mut().rev().find(|c| c.0 == h) {
            Some(c) => c.1 = v,
            None => self.reset(h, v, scope),
        }
    }
}

/* "a 2 b" as (hash of a, 2), (hash of b, dflt); none names nothing. */
pub(super) fn pairs(v: Option<&str>, dflt: i32) -> impl Iterator<Item = (u64, i32)> + '_ {
    let mut toks = v.unwrap_or("").split_whitespace().peekable();
    core::iter::from_fn(move || loop {
        let name = toks.next()?;
        if name.eq_ignore_ascii_case("none") || name.parse::<i32>().is_ok() {
            continue;
        }
        let n = toks.peek().and_then(|t| t.parse::<i32>().ok());
        if n.is_some() {
            toks.next();
        }
        return Some((hash_name(name, true), n.unwrap_or(dflt)));
    })
}

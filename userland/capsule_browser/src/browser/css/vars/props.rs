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

use crate::browser::css::rule::Rule;
use crate::browser::css::rule_index::key::hash_name;

/* One @property registration: whether the property inherits, and its
 * initial value, which stands in wherever it has no declared value. */
pub(in crate::browser::css) struct Prop {
    pub hash: u64,
    pub name: Rc<str>,
    pub inherits: bool,
    pub initial: Option<Rc<str>>,
}

/* The registered custom properties of the sheets, the last registration
 * of a name winning. */
#[derive(Default)]
pub(in crate::browser::css) struct Props {
    list: Vec<Prop>,
}

impl Props {
    pub fn from_rules(rules: &[Rule]) -> Props {
        let mut list: Vec<Prop> = Vec::new();
        for r in rules.iter().filter(|r| r.flags & Rule::PROPERTY != 0) {
            let Some(first) = r.decls.first() else { continue };
            let name: Rc<str> = Rc::from(first.value.as_str());
            let hash = hash_name(&name, true);
            let desc = |n: &str| r.decls.iter().rev().find(|d| d.name == n);
            let inherits = desc("inherits").is_some_and(|d| d.value.trim() == "true");
            let initial = desc("initial-value").map(|d| Rc::from(d.value.trim()));
            list.retain(|p| p.hash != hash || *p.name != *name);
            list.push(Prop { hash, name, inherits, initial });
        }
        Props { list }
    }

    pub fn get(&self, h: u64, name: &str) -> Option<&Prop> {
        if self.list.is_empty() {
            return None;
        }
        self.list.iter().find(|p| p.hash == h && p.name.eq_ignore_ascii_case(name))
    }
}

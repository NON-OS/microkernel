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

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::css::selector::{id_key, name_hash, Pseudo, Simple};

/* Finish a compound for the matcher and the rule index: copy onto it what
 * every argument of one of its :is() or :where() groups requires of the
 * subject (a tag, an id, class names), then hash its class names and id.
 * The copies only restate what the group already demands, so the compound
 * matches the same elements, but the index files a rule like
 * `:is(.nav li)` under li instead of testing it on every element.
 * Specificity was counted before and does not change. */
pub(super) fn keys(s: &mut Simple) {
    hoist(s);
    s.class_keys = s.classes.iter().map(|c| name_hash(b'.', c.as_bytes())).collect();
    s.id_key = s.id.as_deref().map_or(0, |i| id_key(i.as_bytes()));
}

fn hoist(s: &mut Simple) {
    let (mut tag, mut id, mut classes) = (None::<String>, None::<String>, Vec::<String>::new());
    for p in &s.pseudo {
        let Pseudo::Matches(list) = p else { continue };
        let Some((first, rest)) = list.split_first() else { continue };
        let k = &first.key;
        if tag.is_none() && rest.iter().all(|x| x.key.tag == k.tag) {
            tag = k.tag.clone();
        }
        if id.is_none() && rest.iter().all(|x| x.key.id == k.id) {
            id = k.id.clone();
        }
        for c in &k.classes {
            if rest.iter().all(|x| x.key.classes.contains(c)) && !classes.contains(c) {
                classes.push(c.clone());
            }
        }
    }
    if s.tag.is_none() {
        s.tag = tag;
    }
    if s.id.is_none() {
        s.id = id;
    }
    for c in classes {
        if !s.classes.contains(&c) {
            s.classes.push(c);
        }
    }
}

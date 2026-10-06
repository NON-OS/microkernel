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

use alloc::format;
use alloc::rc::Rc;

use crate::browser::css::compute::Author;
use crate::browser::css::parse::parse;
use crate::browser::css::rule::Rule;

use super::text::{closed, TextKey};
use super::CssCache;

/* Parse `more`, the text appended after what `c` holds, alone and add
 * its rules after the kept ones; `key` is the whole text's. The page's
 * rule budget spans every sheet: added rules past what the kept ones
 * left of it are dropped, as the whole text's parse would drop them. */
pub(super) fn append(c: &mut CssCache, more: &str, key: TextKey) {
    let mut rules = core::mem::take(&mut c.author.rules);
    let mut added = parse(more);
    let used = |r: &[Rule]| {
        r.iter().fold((0, 0), |(n, b), r| (n + r.selectors.len().max(1), b + r.cost as usize))
    };
    let (mut n, mut bytes) = used(&rules);
    let fits = added.iter().take_while(|r| {
        (n, bytes) = (n + r.selectors.len().max(1), bytes + r.cost as usize);
        n <= Rule::MAX_SELECTORS && bytes <= Rule::MAX_BYTES
    });
    let keep = fits.count();
    added.truncate(keep);
    rename_anonymous(&mut added, rules.len());
    rules.extend(added);
    c.author = Author::new(rules);
    (c.text, c.memo) = (TextKey { closed: closed(more), ..key }, None);
}

/* An anonymous @layer is named by its offset in the text parsed; in text
 * parsed on its own after `tag` earlier rules the name takes that count
 * too, so it cannot meet an earlier sheet's layer at the same offset. */
fn rename_anonymous(rules: &mut [Rule], tag: usize) {
    let marker = format!("\u{1}{tag}:");
    for r in rules.iter_mut() {
        if let Some(p) = r.layer_name.as_ref().filter(|p| p.contains('\u{1}')) {
            r.layer_name = Some(Rc::from(p.replace('\u{1}', &marker).as_str()));
        }
        if r.flags & Rule::LAYER_ORDER != 0 {
            for d in r.decls.iter_mut() {
                d.value = d.value.replace('\u{1}', &marker);
            }
        }
    }
}

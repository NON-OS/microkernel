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

use crate::browser::dom::Dom;

/* Parent hops the language lookup climbs; a script can build a loop. */
const MAX_HOPS: u32 = 512;

/* :lang(range): the element's language, inherited from the nearest element
 * at or above it with a lang (or xml:lang) attribute, matches the range by
 * RFC 4647 extended filtering, so :lang(fr) holds for fr-CA and :lang(*-CH)
 * for de-CH. An empty lang="" means the language is unknown, which no range
 * matches; so does a document that never states one. */
pub(super) fn lang_matches(dom: &Dom, id: usize, range: &str) -> bool {
    let mut node = id;
    for _ in 0..MAX_HOPS {
        let Some(n) = dom.nodes.get(node) else { return false };
        if let Some(tag) = n.attr("lang").or_else(|| n.attr("xml:lang")) {
            return !tag.is_empty() && extended(range, tag);
        }
        if node == 0 || n.parent == node {
            return false;
        }
        node = n.parent;
    }
    false
}

/* RFC 4647 section 3.3.2: the first subtags agree (or the range's is *),
 * then each later range subtag is found in order among the tag's later
 * subtags without crossing a single-letter extension subtag. */
fn extended(range: &str, tag: &str) -> bool {
    let (mut r, mut t) = (range.split('-'), tag.split('-'));
    match (r.next(), t.next()) {
        (Some("*"), Some(_)) => {}
        (Some(a), Some(b)) if a.eq_ignore_ascii_case(b) => {}
        _ => return false,
    }
    for want in r.filter(|s| *s != "*") {
        loop {
            match t.next() {
                Some(have) if have.eq_ignore_ascii_case(want) => break,
                Some(have) if have.len() == 1 => return false,
                Some(_) => {}
                None => return false,
            }
        }
    }
    true
}

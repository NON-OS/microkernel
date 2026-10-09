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

use crate::browser::css::compute::Author;
use crate::browser::css::parse::parse;
use crate::browser::css::rule_index::RuleIndex;
use crate::browser::css::ua::ua_rules;

use super::append::append;
use super::flips::flips;
use super::text::{closed, fnv, TextKey, FNV_START};
use super::CssCache;

/* The cache holding the rules of `css` at `viewport`, under the viewport
 * guard the caller holds. Same text and viewport: kept. Same text at a
 * viewport where no recorded @media or @container verdict flips: kept.
 * The old text followed by more, when the old text ended between rules:
 * only the addition is parsed. Anything else parses whole. The kept
 * styles go whenever the rules change. */
pub(super) fn ensure<'c>(
    cache: &'c mut Option<CssCache>,
    css: &str,
    viewport: (u32, u32),
) -> &'c mut CssCache {
    let old =
        cache.as_ref().map(|c| c.text.len).filter(|&n| n <= css.len() && css.is_char_boundary(n));
    let cut = old.unwrap_or(0);
    let pre = fnv(FNV_START, &css.as_bytes()[..cut]);
    let key = TextKey { len: css.len(), hash: fnv(pre, &css.as_bytes()[cut..]), closed: false };
    let fresh = match cache.as_mut() {
        Some(c) if (c.text.len, c.text.hash) == (key.len, key.hash) => {
            c.viewport == viewport || !flips(&c.author.rules)
        }
        Some(c)
            if c.viewport == viewport && c.text.closed && old.is_some() && pre == c.text.hash =>
        {
            append(c, &css[cut..], key);
            true
        }
        _ => false,
    };
    let prior_js = cache.as_ref().map(|c| c.js);
    let c = match cache.take() {
        Some(mut c) if fresh => {
            c.viewport = viewport;
            c
        }
        prior => {
            let ua = prior.map(|p| p.ua).unwrap_or_else(|| {
                let rules = ua_rules();
                let index = RuleIndex::build(&rules, false);
                (rules, index)
            });
            let text = TextKey { closed: closed(css), ..key };
            let author = Author::new(parse(css));
            let js = prior_js.unwrap_or((true, None));
            CssCache { text, viewport, ua, author, memo: None, laid: None, js }
        }
    };
    cache.insert(c)
}

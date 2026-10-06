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

//! What a page reached by a short .anyone name must say. A short name is
//! weaker than the full address, since the list the Anyone DNS services sign
//! decides where it points, so the browser shows net.anon's own words next to
//! the address whenever the page's host is one. The test is the route link's
//! own, so the terminal and the browser cannot disagree on which names are
//! short.

use alloc::vec::Vec;

pub use nonos_route_link::SHORT_NOTICE;

/// The notice for the page at `url`, when its host is a short .anyone name.
pub fn notice_for(url: &str) -> Option<&'static str> {
    let parsed = crate::browser::url::parse(url)?;
    nonos_route_link::is_short_anyone(&parsed.host).then_some(SHORT_NOTICE)
}

/// `text` cut at spaces into lines no wider than `width` by `measure`. A
/// word wider than a line stands on a line of its own.
pub fn wrap(text: &str, width: i32, measure: impl Fn(&str) -> i32) -> Vec<&str> {
    let mut lines = Vec::new();
    // The line being built, as a range of `text`, and the start of the next word.
    let mut line: Option<(usize, usize)> = None;
    let mut at = 0usize;
    while at < text.len() {
        let end = text[at..].find(' ').map_or(text.len(), |k| at + k);
        line = match line {
            Some((s, _)) if measure(&text[s..end]) <= width => Some((s, end)),
            Some((s, e)) => {
                lines.push(&text[s..e]);
                Some((at, end))
            }
            None => Some((at, end)),
        };
        at = end + 1;
    }
    if let Some((s, e)) = line {
        lines.push(&text[s..e]);
    }
    lines
}

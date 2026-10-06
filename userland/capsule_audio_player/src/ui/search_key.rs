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

//! Keys on the Search page. The page and the top bar's field were drawn,
//! and a click on the field opened the page, but no key reached the query,
//! so the page only ever showed its empty state.
//!
//! The field also takes the address of an MP3 to download (`fetch`). Words
//! to search fit in a few dozen bytes; an address from a file host runs to
//! hundreds, signed query and all, so a query that begins as an address is
//! given room for one, and Ctrl+V pastes it rather than having it typed.

extern crate alloc;
use alloc::string::String;

use nonos_app_skeleton::input::text::paste_char;
use nonos_app_skeleton::{KEY_BACKSPACE, KEY_ENTER, KEY_ESC};

/// Bytes the query holds; the field shows about this many.
pub const QUERY_MAX: usize = 48;

/// Bytes the query holds once it begins as an address. The field then shows
/// its end, where the file's name is.
pub const ADDRESS_MAX: usize = 1024;

/// What a paste into the query did.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pasted {
    Whole,
    /// It went in only up to what the query holds.
    Cut,
    /// The clipboard held nothing to paste.
    Empty,
}

/// How many bytes `query` may hold.
pub fn room(query: &str) -> usize {
    if query.starts_with("http") { ADDRESS_MAX } else { QUERY_MAX }
}

/// Paste `text` onto the end of the query, a character at a time, as far as
/// the query holds. A tab becomes a space and other control characters are
/// left out, as typing could not enter them.
pub fn paste_query(query: &mut String, text: &str) -> Pasted {
    // Judged on the query as it will read: pasting a whole address into an
    // empty field is what gives it an address's room.
    let mut joined = String::from(query.as_str());
    joined.extend(text.chars().take(4));
    let room = room(&joined);
    let mut any = false;
    for ch in text.chars().filter_map(paste_char) {
        if query.len() + ch.len_utf8() > room {
            return Pasted::Cut;
        }
        query.push(ch);
        any = true;
    }
    if any { Pasted::Whole } else { Pasted::Empty }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SearchKey {
    /// The query changed: the results start from the top again.
    Edited,
    /// Enter: play the first result, as the page's "Play now" does.
    PlayFirst,
    /// Not a search key: the transport keys (the arrows' seek) still work.
    Pass,
}

pub fn search_key(query: &mut String, code: u32) -> SearchKey {
    match code {
        KEY_ENTER => SearchKey::PlayFirst,
        KEY_BACKSPACE if query.pop().is_some() => SearchKey::Edited,
        KEY_ESC if !query.is_empty() => {
            query.clear();
            SearchKey::Edited
        }
        // Typed text stays text when the query is full, so a space there
        // does not start or stop play.
        0x20..=0x7E => {
            if query.len() < room(query) {
                query.push(code as u8 as char);
            }
            SearchKey::Edited
        }
        _ => SearchKey::Pass,
    }
}

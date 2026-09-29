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

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

/// A start or end tag. The name is ASCII lowercase and, when the input
/// already wrote it that way, borrowed from the input rather than copied;
/// the attributes are in source order with duplicates dropped (the first
/// one wins).
pub struct Tag<'a> {
    pub name: Cow<'a, str>,
    pub attrs: Vec<(String, String)>,
    pub self_closing: bool,
}

impl Tag<'_> {
    /// A tag the tree builder makes up, for an element whose tags a page may
    /// leave out (html, head, body, tbody, tr, colgroup, p).
    pub fn implied(name: &'static str) -> Self {
        Tag { name: Cow::Borrowed(name), attrs: Vec::new(), self_closing: false }
    }

    pub fn attr(&self, name: &str) -> Option<&str> {
        self.attrs.iter().find(|(k, _)| k == name).map(|(_, v)| v.as_str())
    }
}

/// A DOCTYPE. A missing name or identifier is None, which the quirks rules
/// tell apart from an empty one.
#[derive(Default)]
pub struct Doctype {
    pub name: Option<String>,
    pub public_id: Option<String>,
    pub system_id: Option<String>,
    pub force_quirks: bool,
}

pub enum Token<'a> {
    Doctype(Doctype),
    Start(Tag<'a>),
    End(Tag<'a>),
    /// A comment or processing instruction. The tree has no node for either,
    /// so only where it stood matters: it still ends a run of table text.
    Comment,
    /// A run of character data, borrowed from the input when nothing in it
    /// was decoded. Never empty, never holding a NUL that came from ordinary
    /// text: that arrives as `Null` because each insertion mode treats it
    /// differently.
    Chars(Cow<'a, str>),
    Null,
    Eof,
}

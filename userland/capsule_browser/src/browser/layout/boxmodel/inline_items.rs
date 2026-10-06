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

pub(super) use super::contexts::inline_parts::{Ink, Lead};
use super::display_list::DisplayList;

/* One atom of an inline formatting context, measured at collect time. */
pub(super) enum InlineItem {
    Word {
        text: String,
        ink: Ink,
        href: Option<String>,
        adv: i32,
        line_h: i32,
        node: usize,
        lead: Lead,
    },
    Image {
        src: String,
        alt: String,
        w: i32,
        h: i32,
        href: Option<String>,
        node: usize,
        fit: crate::browser::css::ObjectFit,
        lead: Lead,
    },
    /* An inline-block or form control laid out at the origin; the line box
     * shifts its whole fragment run into the slot. */
    Atom {
        frags: DisplayList,
        w: i32,
        h: i32,
        lead: Lead,
    },
    Break,
}

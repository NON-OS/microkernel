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

use crate::browser::omnibox::{Damage, Focus, History, LineEdit};

/* Who asked for the pending navigation. A navigation the reader started
 * anywhere but the address bar moves the keyboard to the page; one the
 * browser started itself (redirect, retry, script) leaves focus alone. */
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    Omnibox,
    User,
    Auto,
}

pub struct Chrome {
    /* The address bar's text, caret and selection. */
    pub omnibox: LineEdit,
    pub kbd: Focus,
    /* The document on screen: what Reload loads and Esc restores. */
    pub current_url: String,
    /* The last load started: what a failed load retries. */
    pub last_target: String,
    pub history: History,
    /* Search URL with %s where the query goes. */
    pub search: String,
    /* Horizontal scroll of the address text, in pixels. */
    pub text_off: i32,
    /* The link under the pointer, shown in the status bubble. */
    pub hover_href: Option<String>,
    /* The page stopped short of its end (node cap or cut body). */
    pub truncated: bool,
    /* Generation of the newest navigation, and of the one loading. */
    pub nav_gen: u32,
    pub loading_gen: u32,
    pub origin: Origin,
}

pub struct PaintTrack {
    /* Parts dirtied since the last paint, and the parts the paint about to
     * run draws. */
    pub damage: Damage,
    pub painting: Damage,
    /* Bumped with every recorded change; the tick repaints when it moved
     * past what was painted. */
    pub paint_gen: u32,
    pub painted_gen: u32,
    /* Scroll offset the page pixels on screen were drawn at. */
    pub painted_scroll: u32,
    /* What the last tick left on screen, to see what a tick changed. */
    pub page_print: u64,
    pub chrome_print: u64,
    /* Timers ran since the last timer relayout, and when that was. */
    pub js_dirty: bool,
    pub js_relayout_ms: u64,
    /* DOM fingerprint of the last layout this code knows the DOM for. */
    pub laid_print: Option<(u64, u64)>,
}

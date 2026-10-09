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

use nonos_app_skeleton::{KEY_DOWN, KEY_END, KEY_HOME, KEY_PAGE_DOWN, KEY_PAGE_UP, KEY_UP};

/* Which of the two keyboard targets holds focus. A page form field can
 * only be focused while the page holds it, which keeps the two exclusive. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Focus {
    Omnibox,
    Page,
}

/* Where a click landed. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region {
    /* The address pill, or the home page search bar that mirrors it. */
    Omnibox,
    /* A toolbar button: it acts without taking the caret. */
    Toolbar,
    /* The page, where `field` is the form field the click hit, if any. */
    Page,
    /* The window frame or the settings panel: focus stays where it was. */
    Frame,
}

/* Keyboard focus and the focused page field after a click in `region`.
 * `field` is the field the click hit on the page, or for a toolbar click
 * the field that already had focus, which keeps it. */
pub fn focus_after(region: Region, field: Option<usize>, prev: Focus) -> (Focus, Option<usize>) {
    match region {
        Region::Omnibox => (Focus::Omnibox, None),
        Region::Toolbar | Region::Page => (Focus::Page, field),
        Region::Frame if prev == Focus::Omnibox => (Focus::Omnibox, None),
        Region::Frame => (Focus::Page, field),
    }
}

/* Who takes a key-down that is not a global shortcut. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    Omnibox,
    Field(usize),
    Scroll,
    Page,
}

pub fn route(kbd: Focus, field: Option<usize>, code: u32) -> Route {
    if kbd == Focus::Omnibox {
        return Route::Omnibox;
    }
    if let Some(id) = field {
        return Route::Field(id);
    }
    match code {
        KEY_UP | KEY_DOWN | KEY_PAGE_UP | KEY_PAGE_DOWN | KEY_HOME | KEY_END | 0x20 => {
            Route::Scroll
        }
        _ => Route::Page,
    }
}

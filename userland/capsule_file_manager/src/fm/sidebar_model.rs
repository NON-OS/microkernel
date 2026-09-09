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

extern crate alloc;

use alloc::{string::String, vec::Vec};

use super::layout::SIDE_ROW_H;
use super::screen::Screen;

// A section label is shorter than a navigable row, and sections are parted by a
// gap rather than a rule.
pub const LABEL_H: u32 = 26;
pub const SEC_GAP: u32 = 14;

// Downloads is a directory rather than a surface, so its row navigates Browse.
pub const DOWNLOADS: &str = "/downloads/";

/// Where a sidebar row navigates: a top-level surface, or a directory that the
/// Browse surface opens.
#[derive(Clone)]
pub enum SideHit {
    Screen(Screen),
    Path(String),
}

/// One laid-out sidebar line. A section label carries no `hit` and is inert.
pub struct SideRow {
    pub y: u32,
    pub label: String,
    pub hit: Option<SideHit>,
}

/// The fixed navigation block, in drawn order.
pub const NAV: [(&str, Screen); 4] = [
    ("Home", Screen::Home),
    ("Recents", Screen::Recents),
    ("Shared", Screen::Shared),
    ("Tags", Screen::Tags),
];

/// Accumulates rows against a running y, so a section only ever states its own
/// contents and never its offset.
pub struct Rows {
    pub rows: Vec<SideRow>,
    pub y: u32,
}

impl Rows {
    pub fn label(&mut self, text: &str) {
        self.rows.push(SideRow { y: self.y, label: String::from(text), hit: None });
        self.y += LABEL_H;
    }

    pub fn row(&mut self, label: String, hit: SideHit) {
        self.rows.push(SideRow { y: self.y, label, hit: Some(hit) });
        self.y += SIDE_ROW_H;
    }
}

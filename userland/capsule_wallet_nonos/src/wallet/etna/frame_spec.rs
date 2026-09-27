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

//! What a screen asks its frame for, and where the frame put things.

use super::backdrop::Backdrop;
use super::parts::action::Weight;
use super::rect::Rect;

pub struct FrameSpec<'a> {
    pub number: &'a str,
    pub title: &'a str,
    pub back: bool,
    pub backdrop: Option<Backdrop>,
    pub failure: Option<&'a str>,
    /// Up to three pinned actions, top to bottom.
    pub footer: &'a [(&'a str, Weight, bool)],
    pub status: &'a [&'a str],
}

#[derive(Default)]
pub struct FrameLayout {
    pub back: Option<Rect>,
    pub dismiss: Option<Rect>,
    /// The column left for the screen's own content.
    pub content: Rect,
    pub footer: [Rect; 3],
}

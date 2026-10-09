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

//! Builder table indexed by `Glyph`. Order is load-bearing.

use super::canvas::Sprite;
use super::{arrow, nav, tool, transport, view};

pub type Builder = fn(u32, u32) -> Sprite;

pub const COUNT: usize = 11;

pub const BUILDERS: [Builder; COUNT] = [
    transport::play,
    transport::pause,
    transport::prev,
    transport::next,
    nav::library,
    nav::files,
    tool::video,
    arrow::back,
    view::search,
    view::list,
    view::info,
];

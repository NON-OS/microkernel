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

//! The chrome, rail presets and solver call shared by the cases.

use crate::layout::compute::compute;
use crate::layout::limits::LEFT_RAIL_W;
use crate::layout::types::{Chrome, Layout, Rails};

pub const CHROME: Chrome = Chrome {
    titlebar_h: 28,
    tabstrip_h: 0,
    body_pad_top: 6,
    footer_h: 16,
    text_left: 14,
    row_h: 20,
};

pub const NO_RAILS: Rails = Rails { left: 0 };

pub const BOTH_RAILS: Rails = Rails { left: LEFT_RAIL_W };

pub fn lay(w: u32, h: u32, r: Rails) -> Layout {
    compute(w, h, &CHROME, r)
}

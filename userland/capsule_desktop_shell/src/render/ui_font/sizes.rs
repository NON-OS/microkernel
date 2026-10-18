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

//! The desktop's type sizes, in logical points.

use nonos_toolkit::font::ttf;

/// Body text: menu bar, status cluster, labels, dialogs.
pub const UI_PX: f32 = ttf::MIN_UI_PX;

/// Headings: the launcher title.
pub const TITLE_PX: f32 = 20.0;

/// The brand wordmark, set in the bold face.
pub const BRAND_PX: f32 = 14.0;

/// Menu bar titles.
pub const MENU_PX: f32 = 13.5;

/// The right-hand status cluster and its clock.
pub const STATUS_PX: f32 = 13.0;

/// Desktop icon names.
pub const LABEL_PX: f32 = 12.5;

/// Captions: the second, dimmer line under a desktop icon name.
pub const META_PX: f32 = 11.0;

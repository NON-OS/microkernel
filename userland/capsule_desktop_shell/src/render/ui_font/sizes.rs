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

//! The desktop's type sizes, in logical points, on the brand's ramp
//! (HANDOFF/ui/identity.md, Type). The toolkit draws no proportional text
//! under `ttf::MIN_UI_PX`, 14, so none is asked for here.

/// Body text: dialogs and the Launchpad's labels. `type.body`.
pub const UI_PX: f32 = 17.0;

/// Headings: the launcher title. `type.heading`.
pub const TITLE_PX: f32 = 20.0;

/// The brand wordmark, set in the bold face. `type.caption`.
pub const BRAND_PX: f32 = 14.0;

/// Menu bar titles. `type.caption`.
pub const MENU_PX: f32 = 14.0;

/// The right-hand status cluster and its clock. `type.caption`.
pub const STATUS_PX: f32 = 14.0;

/// Desktop icon names. `type.caption`.
pub const LABEL_PX: f32 = 14.0;

/// The second, dimmer line under a desktop icon name: a caption, told from
/// the name by its colour.
pub const META_PX: f32 = 14.0;

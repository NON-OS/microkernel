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

//! The frame's pure parts, and the scale a window is shown at, mounted as
//! the capsule has them, so each finds its sibling as `super::`.

#[path = "../../../capsule_linux/src/linux/wayland/present/frame/geometry.rs"]
pub mod geometry;
#[path = "../../../capsule_linux/src/linux/wayland/present/frame/paint.rs"]
pub mod paint;
#[path = "../../../capsule_linux/src/linux/wayland/present/resample.rs"]
pub mod resample;
#[path = "../../../capsule_linux/src/linux/wayland/present/scale_math.rs"]
pub mod scale_math;

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
//! The combo PHY PLLs: their dividers for a DisplayPort link rate or an
//! HDMI symbol clock, and the Vol 12 sequences that start, map and stop
//! them (p. 176 to 183).

pub mod dividers;
pub mod dp;
pub mod hdmi;
pub mod lock;
pub mod map;
pub mod sequence;
pub mod split;

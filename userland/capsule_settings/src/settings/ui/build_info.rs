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

//! What the Updates screen reports about this build. The values come from the
//! tree the capsule was compiled from, so the screen states what actually
//! shipped.

pub const VERSION: &str = include_str!("../../../../../VERSION");
pub const GIT_SHA: &str = env!("SETTINGS_GIT_SHA");
// The compiler's own `rustc -V` and cargo's target, read by build.rs.
pub const TOOLCHAIN: &str = env!("SETTINGS_RUSTC");
pub const ARCHITECTURE: &str = env!("SETTINGS_ARCH");

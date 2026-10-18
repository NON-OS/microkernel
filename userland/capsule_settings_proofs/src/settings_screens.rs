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

//! The capsule's `settings::screens`, less the shell calls and the keys.

#[path = "../../capsule_settings/src/settings/screens/arrange.rs"]
pub mod arrange;
#[path = "../../capsule_settings/src/settings/screens/keep.rs"]
pub mod keep;
#[path = "../../capsule_settings/src/settings/screens/levels.rs"]
pub mod levels;
#[path = "../../capsule_settings/src/settings/screens/percent.rs"]
pub mod percent;
#[path = "../../capsule_settings/src/settings/screens/text.rs"]
pub mod text;
#[path = "../../capsule_settings/src/settings/screens/wire.rs"]
pub mod wire;

pub use arrange::ScreenPage;

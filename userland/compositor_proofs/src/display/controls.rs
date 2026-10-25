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

//! The display controls the compositor carries to the driver: brightness
//! and a display's inputs. Mounted apart so the display mounts stay short;
//! the rules reach the layout through the re-export below.

pub use super::layout_rule;

#[path = "../../../compositor/src/display/brightness_get.rs"]
pub mod brightness_get;
#[path = "../../../compositor/src/display/brightness_list.rs"]
pub mod brightness_list;
#[path = "../../../compositor/src/display/brightness_rule.rs"]
pub mod brightness_rule;
#[path = "../../../compositor/src/display/input_list.rs"]
pub mod input_list;

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

/*
 * The installer as the whole screen, in first-boot setup's look: setup's
 * panel on the left with the steps, the screen on the right, so the
 * install reads as the rest of the setup it follows.
 */

mod chrome;
mod frame;
pub mod layout;
mod palette;
mod proofs_subtitle;
mod steps;
mod subtitle;

pub use frame::paint;
pub use palette::BACKDROP;

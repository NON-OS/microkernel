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

mod adopt;
mod attach_primary;
mod canvas;
mod canvas_floor;
mod canvas_release;
mod canvas_scale;
mod discover;
mod gop_backing;
mod heads;
mod panel_mm;
mod poll;
mod poll_heads;
mod prime;
mod prime_gop;
mod prime_once;
mod prime_wire;
mod resize;
mod upgrade;

pub use canvas::fit_canvas;
pub use discover::{displays, lookup};
pub use poll::poll;
pub use prime::run_display;
pub use prime_gop::run_gop_once as run_gop;
pub use upgrade::upgrade_from_gop;

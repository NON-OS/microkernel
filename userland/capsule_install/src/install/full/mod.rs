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
 * The installer as the whole screen, on an install boot: straight after
 * first-boot setup, with no window manager, shell or app running, drawn
 * through the compositor the way setup draws. The screens, the plan, the
 * write and the read-back are the window's own; only the frame and the way
 * keys arrive differ.
 */

mod active;
mod draw;
mod grab;
mod input;
mod peers;
mod run;
mod start;
mod surface;

pub use active::{active, setup_kept, wanted};
pub use run::run;

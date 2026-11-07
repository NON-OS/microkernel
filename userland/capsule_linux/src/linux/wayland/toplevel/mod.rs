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

//! `xdg_toplevel`'s modes (modes.rs), asking the guest for one (ask.rs),
//! its first configure (first.rs) and the same told again (again.rs), what
//! it may fill and do (bounds.rs), minimising, moving and resizing it
//! (minimize.rs, moving.rs, resize.rs), its title (title.rs), unmapping it (unmap.rs), and its
//! destroy (destroy.rs).

mod again;
mod ask;
mod bounds;
mod current;
mod destroy;
mod first;
mod frame_max;
mod minimize;
mod modes;
mod moving;
pub mod resize;
mod said;
mod unmap;
pub mod size;
mod title;

pub use again::again;
pub use ask::held;
pub use current::geometry;
pub use destroy::destroy;
pub use first::first;
pub use frame_max::maximize;
pub use minimize::minimize;
pub use moving::moving;
pub use said::said;
pub use title::title;
pub use unmap::unmap;
pub use modes::{has_toplevel, set, toggle, unset};

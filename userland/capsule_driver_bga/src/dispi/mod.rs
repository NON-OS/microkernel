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

mod check;
mod current;
mod dispi_off;
mod endian;
mod fit;
mod restore;
mod set_mode;
mod state;
mod unblank;

pub use check::identify;
pub use current::{current, DispiMode};
pub use endian::little_endian;
pub use fit::video_memory;
pub use restore::restore;
pub use set_mode::{regs_len, set_mode};
pub use state::{read, Dispi};

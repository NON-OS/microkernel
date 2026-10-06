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

//! Controller bring-up: the LPSS wrapper, the DesignWare core checks, the
//! SCL timing and the FIFO depths.

mod bring_up;
mod bus_setup;
mod fifo;
mod init_state;
mod lpss_init;
mod program_clock;
pub mod scl;
mod unlisted;

pub use bring_up::bring_up;
pub use bus_setup::BusSetup;
pub use fifo::fifo_depths;
pub use init_state::InitState;
pub use unlisted::proves_lpss_i2c;

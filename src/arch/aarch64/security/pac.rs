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

//! Return addresses signed with the IA key (FEAT_PAuth, `pac-ret`): one
//! kernel key per boot shared by every CPU, and one per process for EL0.

mod install;
mod prepare;
mod report;
mod return_address;
mod rndr;
mod secret;
mod slot;
mod state;
mod user;

pub(crate) use install::install_kernel_key;
pub use prepare::{bind_this_cpu, prepare, secondary_key};
pub use report::describe;
pub use return_address::{authenticate_return_address, sign_return_address};
pub use secret::Source;
pub use state::active;
pub use user::enter_process;

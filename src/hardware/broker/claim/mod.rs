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

mod claim;
mod firmware;
mod held;
mod leaving;
mod lookup;
mod no_snoop;
mod port_only;
pub(super) mod quiesce;
mod refusal;
mod release;
mod reset;
mod reset_function;
mod reset_io;
mod reset_restore;
mod reset_rule;
mod reset_run;
mod reset_wait;
mod retire;
mod retire_io;
mod scope_check;
mod state;
mod still_held;
mod types;

pub use claim::claim;
pub use held::quiesce_all;
pub use lookup::lookup;
pub(crate) use release::release_as;
pub use release::{release, release_all_for_pid};
pub(crate) use still_held::record_while_held;
pub use types::{Claim, ClaimError};

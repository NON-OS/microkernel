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
mod held;
mod lookup;
mod no_snoop;
mod quiesce;
mod release;
mod scope_check;
mod state;
mod types;

pub use claim::claim;
pub use lookup::lookup;
pub use held::{quiesce_all, quiesce_held};
pub use release::{release, release_all_for_pid};
pub use types::{Claim, ClaimError};

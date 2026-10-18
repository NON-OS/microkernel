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

//! Which hypervisor, if any, this kernel runs under. The classification is
//! pure so the proof crate runs it against every signature; only `probe` and
//! `report` touch the machine.

mod backdoor;
mod backdoor_rule;
mod balloon;
mod classify;
mod guest;
pub mod hv;
mod leaves;
mod probe;
mod refine;
mod report;
mod state;
mod vendor;

pub use backdoor::{backdoor, backdoor_present, BackdoorError};
pub use classify::classify;
pub use guest::Guest;
pub use leaves::{Leaves, LEAF_BASE};
pub use refine::refine;
pub use report::report;
pub use state::{current, hyperv_abi};
pub use vendor::Hypervisor;

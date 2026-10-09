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

mod budget;
mod count;
mod drain;
mod log;
mod poll;
mod reason;
mod record;
mod status;
mod take;

pub use budget::{Budget, LINES_PER_POLL};
pub use count::fault_total;
pub use drain::drain_faults;
pub use poll::poll_faults;
pub use reason::{bdf_text, is_interrupt, reason_text};
pub use record::FaultRecord;

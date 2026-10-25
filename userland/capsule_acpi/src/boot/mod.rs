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

//! From the kernel's tables to a running platform.

mod assemble;
mod bringup;
mod bringup_say;
mod eval;
mod first;
mod ged;
mod late;
mod load;
mod load_rule;
mod osc;
mod osc_host;
mod pass;
mod report;
mod report_prt;
mod say;
mod sci;
mod sci_claim;
mod start;
mod state;
mod tables;

use crate::sleep::shutdown_say::Part;

pub use eval::{evaluate, evaluator};
pub use osc_host::NATIVE_HOT_PLUG_ON;
pub use report::path;
pub use start::boot;
pub use state::Acpi;

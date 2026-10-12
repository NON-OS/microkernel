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

//! The interpreter: evaluating methods and expressions (chapter 19's
//! operators) under a budget.

mod body;
mod budget;
mod call;
mod data;
mod define;
mod deref;
mod element;
mod evaluate;
mod fault;
mod field;
mod flow;
mod frame;
mod host;
mod limits;
mod load_exec;
mod machine;
mod name;
mod named;
mod operand;
mod ops;
mod path_text;
mod personality;
mod place;
mod predefined;
mod region_state;
mod result;
mod runtime;
mod stmt;
mod term;
mod terms;
mod var;
mod write;
mod write_named;
mod write_ref;

pub use fault::{Fault, FaultKind};
pub use flow::Flow;
pub use frame::{Frame, Slot};
pub use host::Host;
pub use limits::ExecLimits;
pub use machine::Machine;
pub use personality::Personality;
pub use region_state::{BufferField, PciPath, Region, RegionAccess};
pub use runtime::{Holder, Runtime};
pub use term::REVISION;

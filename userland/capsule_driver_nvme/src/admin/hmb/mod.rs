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

//! The Host Memory Buffer a DRAM-less controller asks for (NVMe 1.4,
//! 5.21.1.13): how much to give it and in what pieces, the descriptor list
//! that names them, and the rules that keep it from costing the disk. Pure,
//! for the host proofs.
//!
//! A DRAM-less SSD (Samsung PM991, WD SN530 and SN740, SK hynix BC711,
//! Kioxia BG4 and the like) keeps its mapping tables in host memory when the
//! host offers it. The spec lets such a controller run without one, but
//! that is a path the firmware sees little of: Linux always offers it
//! (nvme_setup_host_mem), and so does Windows. This driver does too.

mod ask;
mod budget;
mod layout;
mod plan;

pub use ask::HmbAsk;
pub use budget::{ends_attempt, extras_allowed, ENABLE_TIMEOUT_MS, QUEUES_TIMEOUT_MS};
pub use layout::{descriptor, enable_dwords, DESCRIPTOR_BYTES, ONE_QUEUE_PAIR};
pub use plan::{plan, HmbPlan, MAX_DESCRIPTORS, PAGE};

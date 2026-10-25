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

//! `git clone` as a terminal job. The clone is nonos_git's own steps in
//! order (the ref advertisement, the pack, the write into the store), each
//! run against a transport that leaves its request for the job
//! (`git::transport::ask`); the job carries it over the chosen network a
//! tick at a time and runs the step again with the answer. The window keeps
//! painting through a transfer of tens of megabytes, says how far it has
//! got, and Ctrl+C ends it with the connection closed and nothing written.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use nonos_git::ObjectId;
use nonos_route_link::Route;

use crate::git::{Stepped, VfsStorage};
use crate::mixnet::Exchange;

#[path = "job_ask.rs"]
mod job_ask;
#[path = "job_carry.rs"]
mod job_carry;
#[path = "job_prepare.rs"]
mod job_prepare;
#[path = "job_report.rs"]
mod job_report;
#[path = "job_steps.rs"]
mod job_steps;
#[path = "job_write.rs"]
mod job_write;

pub use job_prepare::prepare;

/// A clone stops at the tip by default. Whole histories are large, so the
/// depth is stated rather than left to run.
const DEPTH: u32 = 1;
const COMMAND: &str = "git clone";

enum Step {
    Refs,
    Pack(ObjectId),
    /// The pack is here; the next tick writes it, after this one has said so.
    Write(ObjectId, Vec<u8>),
}

pub struct CloneJob {
    transport: Stepped,
    storage: VfsStorage,
    route: Route,
    rtc: u64,
    url: String,
    into: String,
    branch: String,
    step: Option<Step>,
    /// The request being carried, and the exchange carrying it.
    carrying: Option<(Vec<u8>, Exchange)>,
    reported: usize,
}

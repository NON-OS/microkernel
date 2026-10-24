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

//! The kernel calls the demo's apps make, answered from one fixed machine
//! (`machine`) whose display mode and driver a test sets, so every screen is
//! drawn with values as long as a real boot's. The libc files that make no
//! call (the procstat header, the worker handoff, the deadline) are mounted
//! as they are.

// Every call keeps libc's signature, raw pointers taken by safe functions.
#![allow(clippy::not_unsafe_ptr_arg_deref)]

mod attest;
pub mod boot_attest;
mod calls;
mod jobs;
mod machine;
mod proc;
mod proc_types;
mod quiet;
mod seat;
mod surface;
mod time;

#[path = "../../../libc/src/thread/handoff.rs"]
mod handoff;
#[path = "../../../libc/src/procstat_header.rs"]
pub mod procstat_header;

pub use attest::{
    mk_attest_doc, mk_attest_entries, mk_attest_status, AttestStatus, ATTEST_ENTRY_LEN,
};
pub use calls::{
    mk_cap_check, mk_getpid, mk_ipc_call_timeout, mk_service_lookup, mk_time_millis, mk_uptime_ms,
    nonos_display_dimensions,
};
pub use handoff::{Handoff, Look};
pub use jobs::*;
pub use machine::{show, OWN_PID};
pub use proc::{
    mk_proc_stat, ProcStatEntry, ProcStatHeader, PROC_FLAG_DEBUG_WITHHELD, PROC_NAME_LEN,
};
pub use quiet::*;
pub use seat::WorkerSeat;
pub use surface::*;
pub use time::{mk_time_rtc, Deadline, RtcTime};

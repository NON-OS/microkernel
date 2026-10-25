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

//! The hardware report, version 1: one line a fact, each opened by its kind.
//!
//! ```text
//! NONOS-HWREPORT 1
//! build <commit> <profile>
//! posture <confined|dma-unconfined>
//! pci <vendor>:<device> class=<cc>.<ss>.<pi> claimed=<0|1> external=<0|1>
//! acpi <SIG> <instance> <length> <blake3 hex, or - for a table never hashed>
//! log <a boot log line, scrubbed>
//! end <the number of lines before this one>
//! ```
//!
//! What the report never carries is decided here, not by its caller: a log
//! line is scrubbed (redact.rs), an ACPI table outside the allowlist goes
//! without its digest (acpi.rs), and the PCI line has no field for a serial,
//! a bus address or a BAR.

#![no_std]

pub mod acpi;
pub mod parse;
mod parse_fields;
mod parse_line;
mod parse_rows;
pub mod pci;
pub mod redact;
mod redact_runs;
mod redact_shapes;
mod redact_words;
pub mod report;
mod report_head;
pub mod sink;

/// The first line of every version 1 report.
pub const HEADER: &[u8] = b"NONOS-HWREPORT 1";

/// The longest line a report may hold, header and kind included.
pub const MAX_LINE: usize = 240;

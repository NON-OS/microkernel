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

//! What a `pkg` request asks of the installer, and what it says once
//! answered. The asking runs on a worker thread (`job.rs`) or, where there is
//! none, inline (`run.rs`); the saying runs on the window thread either way.

use alloc::vec::Vec;

use super::args;
use super::summary::PkgSummary;
use crate::term::cwd::resolve;

#[path = "work_perform.rs"]
mod work_perform;
#[path = "work_report.rs"]
mod work_report;

pub(super) use work_perform::perform;
pub(super) use work_report::report;

pub(super) const USAGE: &[u8] = b"usage: nox pkg install <path> [--yes] | remove <name> | status";

/// A request the installer answers.
pub(super) enum PkgOp {
    /// Verify the package and show what it would be granted; with `yes`,
    /// install it too.
    Install { path: Vec<u8>, yes: bool },
    /// Drop an installed slug.
    Remove { name: Vec<u8> },
}

/// The installer's answer to a `PkgOp`.
pub(super) enum PkgDone {
    QueryFailed(i32),
    /// Verified; installing waits for the person's --yes.
    Queried(PkgSummary),
    /// Verified, then the commit's outcome.
    Committed(PkgSummary, Result<(), i32>),
    Removed(Vec<u8>, Result<(), i32>),
}

/// `install <path> [--yes]` or `remove <name>`, or None for anything else
/// (`status`, or a usage the caller reports).
pub(super) fn parse(cwd: &[u8], args: &[&[u8]]) -> Option<PkgOp> {
    match args.split_first() {
        Some((&b"install", rest)) => {
            let (raw, yes) = args::install(rest)?;
            Some(PkgOp::Install { path: resolve(cwd, raw), yes })
        }
        Some((&b"remove", [name, ..])) => Some(PkgOp::Remove { name: name.to_vec() }),
        _ => None,
    }
}

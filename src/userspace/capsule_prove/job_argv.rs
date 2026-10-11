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

//! What the headless prover is told at spawn (plan/15): that it is the job,
//! which op it serves, and the one pid whose frame it takes. The capsule
//! reads these from argv; nothing a caller sends can widen them.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// The argv the capsule reads to serve one job, not open a window.
pub const JOB_ARG: &str = "--nonos-prove-job";

#[derive(Clone, Copy, Debug)]
pub enum JobKind {
    Prove,
    Deliver,
}

/// `--nonos-prove-job`, then `prove` or `deliver`, then the pid in decimal.
pub(super) fn job_argv(kind: JobKind, pid: u32) -> Vec<String> {
    let k = match kind {
        JobKind::Prove => "prove",
        JobKind::Deliver => "deliver",
    };
    alloc::vec![String::from(JOB_ARG), String::from(k), pid.to_string()]
}

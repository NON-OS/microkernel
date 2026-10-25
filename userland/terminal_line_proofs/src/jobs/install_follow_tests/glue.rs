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

//! The job asks after its own load by name, a refusal is said as a quick one
//! is, and the probe's op is the installer's.

const JOB: &str =
    include_str!("../../../../capsule_terminal/src/command/builtin/nox/install/job_follow.rs");
const PROBE: &str =
    include_str!("../../../../capsule_terminal/src/command/builtin/nox/install/probe.rs");
const OPS: &str = include_str!("../../../../capsule_installer/src/protocol/types.rs");

fn flat(s: &str) -> String {
    s.split_whitespace().collect()
}

#[test]
fn the_job_asks_after_its_own_load_and_says_a_refusal_as_install_does() {
    let job = flat(JOB);
    assert!(job.contains("follow.probed(now,probe(&self.stem));"));
    assert!(job.contains("Step::Refused(status)=>{self.load=None;emit_err(out,status);"));
}

#[test]
fn the_probe_asks_the_installer_for_the_outcome_by_its_op() {
    assert!(PROBE.contains("const OP_LOAD_OUTCOME: u16 = 9;"));
    assert!(OPS.contains("pub const OP_LOAD_OUTCOME: u16 = 9;"));
    assert!(flat(PROBE).contains("tx.push(name.len()asu8);tx.extend_from_slice(name);"));
}

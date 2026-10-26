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

//! One record, one consume: a RECEIPT job peeks a record and leaves it sealed
//! until its ack, so nothing else may take that record meanwhile. The only
//! taker is a prover, the window or a job, and one prover runs at a time:
//! both are spawned only from init's one loop, one after the other, each
//! spawn registers its endpoint before it returns, and each refuses while
//! the other's is registered. A RECEIPT job is a prover like any other, so
//! no PROVE or DELIVER job, and no window, starts while it is alive
//! (exclusion_loop_tests.rs: where they are spawned from).

const JOB: &str = include_str!("../../../../src/userspace/capsule_prove/job.rs");
const WINDOW: &str = include_str!("../../../../src/userspace/capsule_prove/spawn.rs");
const QUEUE: &str = include_str!("../../../../src/userspace/init/prove_job/queue.rs");

#[test]
fn a_live_prover_is_a_job_or_the_window() {
    let live = "pub fn prover_live() -> bool {\n    lookup_service(JOB).is_some()\n        \
                || lookup_port(JOB_PORT).is_some()\n        || lookup_service(WINDOW).is_some()\n}";
    assert!(JOB.contains(live));
}

#[test]
fn no_job_spawns_and_none_is_queued_out_while_a_prover_lives() {
    let refused =
        "    if prover_live() {\n        return Err(SpawnError::EndpointCollision);\n    }";
    let spawn = &JOB[JOB.find("pub fn spawn_prove_job(").unwrap()..];
    assert!(spawn[..spawn.find("spawn_verified").unwrap()].contains(refused));
    assert!(QUEUE.contains("!prover_live() && QUEUE.lock().ask("));
    let service = &QUEUE[QUEUE.find("pub(crate) fn service()").unwrap()..];
    assert!(
        service.starts_with("pub(crate) fn service() {\n    if prover_live() {\n        return;")
    );
}

#[test]
fn no_window_spawns_while_a_job_lives() {
    let window = &WINDOW[WINDOW.find("pub fn spawn_prove_instance()").unwrap()..];
    let refused =
        "if crate::services::registry::lookup_service(super::job::JOB).is_some() {\n        \
                   return Err(SpawnError::EndpointCollision);";
    assert!(window[..window.find("spawn_next_instance(").unwrap()].contains(refused));
}

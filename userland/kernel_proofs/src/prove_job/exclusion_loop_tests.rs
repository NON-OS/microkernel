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

//! A prover, job or window, is spawned only from init's one loop, the window
//! first and then the job, never both at once and from nowhere else: so the
//! check each makes for the other (exclusion_tests.rs) is never raced.

const QUEUE: &str = include_str!("../../../../src/userspace/init/prove_job/queue.rs");
const SPAWNS: &str = include_str!("../../../../src/userspace/init/instance_spawn/service.rs");
const LOOP: &str = include_str!("../../../../src/userspace/init/supervisor/loop_impl.rs");
const OWN: [&str; 4] = [
    "capsule_prove/job.rs",
    "capsule_prove/spawn.rs",
    "prove_job/queue.rs",
    "instance_spawn/service.rs",
];

#[test]
fn both_spawn_only_from_inits_one_loop() {
    assert_eq!(QUEUE.matches("spawn_prove_job(").count(), 3);
    assert_eq!(SPAWNS.matches("spawn_prove_instance()").count(), 1);
    let at = |s| LOOP.find(s).unwrap();
    let (window, job) = (at("service_instance_spawns();"), at("service_prove_job();"));
    assert!(window < job && LOOP[window..job].lines().count() < 6, "one loop, in turn");
    for f in walk(concat!(env!("CARGO_MANIFEST_DIR"), "/../../src")) {
        let text = std::fs::read_to_string(&f).unwrap_or_default();
        let named = text.contains("spawn_prove_job(") || text.contains("spawn_prove_instance()");
        assert!(!named || OWN.iter().any(|e| f.ends_with(e)), "{f} spawns a prover elsewhere");
    }
}

fn walk(dir: &str) -> Vec<String> {
    let mut out = Vec::new();
    for e in std::fs::read_dir(dir).unwrap().flatten() {
        let p = e.path().to_string_lossy().into_owned();
        match e.file_type().unwrap().is_dir() {
            true => out.extend(walk(&p)),
            false if p.ends_with(".rs") => out.push(p),
            false => {}
        }
    }
    out
}

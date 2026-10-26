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

//! The worker's progress: the result is read only once a final stage is
//! published, exactly once, and never half-written, by a real second thread
//! as by this one; and the publish is ordered as progress.rs says.

use crate::job::progress::Progress;
use nonos_proof_abi::wire::Stage;

#[test]
fn the_result_is_read_only_on_a_final_stage_and_once() {
    let p: Progress<Vec<u8>> = Progress::new();
    assert_eq!((p.stage(), p.take()), (Stage::Queued, None));
    for s in [Stage::Device, Stage::Isolation] {
        p.advance(s);
        assert_eq!((p.stage(), p.take()), (s, None), "{s:?} is no final stage");
    }
    p.finish(true, vec![7; 3]);
    assert_eq!((p.stage(), p.take()), (Stage::Done, Some(vec![7; 3])));
    assert_eq!(p.take(), None, "taken once");
    let failed: Progress<i32> = Progress::new();
    failed.finish(false, -5);
    assert_eq!((failed.stage(), failed.take()), (Stage::Failed, Some(-5)));
}

/// A worker thread publishing a large result while this thread polls: the
/// result it takes is always whole.
#[test]
fn a_result_taken_across_threads_is_always_whole() {
    for round in 0..200u32 {
        let p: &'static Progress<Vec<u32>> = Box::leak(Box::new(Progress::new()));
        let worker = std::thread::spawn(move || {
            p.advance(Stage::Device);
            p.finish(true, (0..4096).map(|i| i ^ round).collect());
        });
        let got = loop {
            if let Some(v) = p.take() {
                break v;
            }
            std::hint::spin_loop();
        };
        worker.join().unwrap();
        assert!(got.iter().enumerate().all(|(i, &v)| v == i as u32 ^ round), "round {round}");
    }
}

/// The result is written before the final stage is stored, both stores are
/// Release and the load Acquire: what makes the take above sound on every
/// architecture, not only on this host's.
#[test]
fn the_publish_is_ordered() {
    let src = include_str!("../../../capsule_prove/src/job/progress.rs");
    let finish =
        &src[src.find("pub fn finish").unwrap()..src.find("/// The stage the worker").unwrap()];
    let write = finish.find("*self.result.get() = Some(result)").expect("the result written");
    let store = finish.find("self.stage.store(last, Ordering::Release)").expect("a Release store");
    assert!(write < store, "the result is written before the stage is published");
    assert!(src.contains("self.stage.store(stage as u32, Ordering::Release)"));
    assert!(src.contains("self.stage.load(Ordering::Acquire)"));
    assert!(!src.contains("Ordering::Relaxed"));
}

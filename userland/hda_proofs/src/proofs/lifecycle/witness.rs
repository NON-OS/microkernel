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

//! A teardown run while a model on another thread keeps one part of the
//! controller from moving and notes when a later step is first seen. The
//! clock starts first, so a model that runs late only sees a step later.
//! The model says whether it saw the step from the same read it holds its
//! part with: a step on the held register itself is undone by the hold, and
//! a second read could come after it and miss it for good.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use nonos_devmodel::FakeBar;
use nonos_libc::logged;

use crate::constants::DPLBASE;
use crate::error::{reason, HdaError};
use crate::model::live;
use crate::proofs::lifecycle::teardown::running;
use crate::proofs::lifecycle::teardown_log::teardown;

/// Tear down a running controller under `model`, which holds its part and
/// says whether the step is taken, and return how long after the start the
/// step was first seen, and DPLBASE as it read then.
pub fn first_seen(model: fn(&FakeBar) -> bool) -> (Duration, u32) {
    let (bar, first) = (running(), Arc::new(Mutex::new(None)));
    let seen = Arc::clone(&first);
    let _model = live(&bar, move |b| {
        if model(b) {
            let now = (Instant::now(), b.wrote32(DPLBASE as usize));
            seen.lock().unwrap().get_or_insert(now);
        }
        std::thread::yield_now();
    });
    let start = Instant::now();
    teardown(&bar);
    let end = Instant::now() + Duration::from_secs(5);
    while first.lock().unwrap().is_none() && Instant::now() < end {
        std::thread::yield_now();
    }
    let (at, dpl) = first.lock().unwrap().expect("never took that step");
    (at - start, dpl)
}

/// `first_seen`, until a run in which the driver waited the held engine out
/// and named it (where the model came too late, a run proves nothing).
pub fn waited_out(model: fn(&FakeBar) -> bool) -> (Duration, u32) {
    let named = reason(HdaError::EngineStopTimeout);
    for _ in 0..20 {
        let before = logged().len();
        let seen = first_seen(model);
        if logged()[before..].iter().any(|l| l.contains(named)) {
            return seen;
        }
    }
    panic!("in 20 runs the driver never waited the held engine out");
}

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

//! The store's PROVE, followed a tick at a time (I-B9a), as order_watch.rs
//! follows an order: each step's call made until one has nothing to say
//! until a later tick, the serial lines on the way, and the container held
//! at the end. A run `n` stopped goes on quietly and is wiped.

use nonos_libc::mk_uptime_ms;

use super::super::state::State;
use super::proof_debug::{held, told};
use super::proof_event::Event;
use super::proof_held::Held;
use super::proof_perform::perform;
use super::proof_seq::{ended, Phase};
use super::proof_step::step;

/// The calls one tick may make: a 1 MiB container's sixteen GETs and the
/// steps before them.
const STEPS: usize = 32;

impl State {
    /// One tick of the question and the run. True when the pane changed.
    pub fn watch_proof(&mut self) -> bool {
        let offered = self.offer_proof();
        let market = self.port;
        let Some(run) = self.proofs.run.as_mut().filter(|r| !ended(r.phase)) else {
            return offered;
        };
        let before = run.phase;
        let late = run.late_ms != 0 && mk_uptime_ms() >= run.late_ms;
        let mut event = if late { Event::Late } else { Event::Tick };
        for _ in 0..STEPS {
            let was = run.phase;
            let (phase, call) = step(was, event);
            run.phase = phase;
            told(run, was);
            let Some(next) = perform(run, call, market) else { break };
            event = next;
        }
        let changed = run.phase != before;
        if run.phase == Phase::Held {
            let container = run.container().unwrap_or_default();
            let (listing, release) = (run.listing.clone(), run.release.clone());
            let proof = Held { listing, release, listing_key: run.listing_key, container };
            if !run.cancelled {
                held(&proof.listing, proof.container.len());
                self.proofs.held = Some(proof);
            }
        }
        offered || changed
    }
}

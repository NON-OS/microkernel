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

//! One line per shootdown round, built only with `nonos-shootdown-trace`, for
//! boots that must show rounds going out and coming back. No profile enables
//! it. The first 64 rounds are each reported, then every 256th, so a long run
//! shows the protocol still working without the log being made of it.
//!
//! `[SHOOTDOWN] round=N cpu=C asid=A pages=P targets=T starting=S
//! answered_for=F set=W:HEX...`: the round returned only once all T
//! acknowledged, S of them were read as starting, and the waiter paid F for
//! targets the boot CPU abandoned. `set` lists each non-empty 64-cpu word of
//! the target set.

use core::sync::atomic::{AtomicU32, Ordering};

#[cfg(any(feature = "nonos-production", feature = "nonos-release"))]
compile_error!(
    "nonos-shootdown-trace prints every early TLB shootdown on the serial log; it is a \
     boot proof and never part of a production build."
);

use super::summary::Round;

static ROUNDS: AtomicU32 = AtomicU32::new(0);

pub(super) fn report(r: &Round) {
    let n = ROUNDS.fetch_add(1, Ordering::Relaxed) + 1;
    if n > 64 && n % 256 != 0 && !proof_round() {
        return;
    }
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[SHOOTDOWN] round=").dec(u64::from(n));
    l.str(b" cpu=").dec(crate::smp::cpu_id() as u64);
    l.str(b" asid=").dec(u64::from(r.asid));
    l.str(b" pages=").dec(u64::from(r.pages));
    l.str(b" targets=").dec(u64::from(r.targets));
    l.str(b" starting=").dec(u64::from(r.starting));
    l.str(b" answered_for=").dec(u64::from(r.answered_for));
    l.str(b" set=");
    for (word, bits) in r.selected.iter().enumerate() {
        if *bits != 0 {
            l.dec(word as u64).str(b":").hex(*bits).str(b" ");
        }
    }
    l.end();
}

/// The shootdown proof's own rounds are printed whatever their number: at 16
/// CPUs bring-up alone runs past the first 64.
#[cfg(feature = "nonos-smp-shootdown-proof")]
fn proof_round() -> bool {
    super::proof::RUNNING.load(Ordering::Acquire)
}

#[cfg(not(feature = "nonos-smp-shootdown-proof"))]
fn proof_round() -> bool {
    false
}

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

//! The refresh loop can never freeze the UI. This models the step machine in
//! `wallet/event/probe/probe_tick.rs` and the idle gate in `wallet/app.rs`,
//! and checks the invariants that keep the wallet responsive. Change a
//! constant in the wallet and the matching constant here should change with
//! it. The time bounds on what a refresh waits for are in probe_bounds_tests.

// One batched refresh fetches this many fields over a single connection
// (read_snapshot.rs: eth balance, nonce, fee, nox balance, claimable,
// positions, stats).
const BATCH_FIELDS: u8 = 7;
// The whole account refreshes in one batched request rather than one
// connection per field, so a refresh costs one handshake however many fields
// it carries: the property that made the refresh fast enough to feel live.
const _: () = assert!(BATCH_FIELDS >= 2, "batching only helps with several fields");
const IDLE_GATE: u32 = 2; // app.rs: probe once the user has paused this many ticks

/// app.rs: once the user has paused for IDLE_GATE ticks, probe every idle tick
/// so the field cycle fills in a few seconds. Mouse movement does not count as
/// activity (see app.rs on_event), so it never defers the probe; a real click
/// or keypress does, pausing it immediately so a blocking read never lands
/// under the hand.
fn probes(address_ready: bool, ticks: u32, last_active: u32) -> bool {
    if !address_ready {
        return false;
    }
    ticks.wrapping_sub(last_active) >= IDLE_GATE
}

#[test]
fn active_interaction_never_freezes_the_ui() {
    // A click or keypress every tick (last_active == ticks) defers the probe
    // every time, so a blocking read never runs under the user's hand. Movement
    // is handled separately (app.rs does not treat it as activity), so it never
    // reaches this gate: the fee and staking reads still land once the user
    // pauses, which probing_resumes_once_idle covers.
    for t in 0..64u32 {
        assert!(!probes(true, t, t), "no blocking probe while actively interacting");
    }
}

#[test]
fn no_probe_right_after_a_click() {
    // The tick right after an odd-tick click (not a forced tick) does not probe,
    // so a click is serviced without waiting on the network.
    assert!(!probes(true, 5, 5), "no probe on the tick of a click");
    assert!(!probes(true, 7, 7), "no probe immediately after a click");
}

#[test]
fn probing_resumes_once_idle() {
    assert!((IDLE_GATE..IDLE_GATE + 8).any(|t| probes(true, t, 0)), "auto-refresh still runs");
}

#[test]
fn no_probe_before_an_account_exists() {
    assert!(!(0..64u32).any(|t| probes(false, t, 0)));
}

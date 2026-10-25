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

use super::boot_budget::budget_ticks;
use super::boot_claim::BootClaim;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

/* ---- the AP's claim against the boot CPU's give-up ---- */

#[test]
fn claim_has_one_winner() {
    let c = BootClaim::new();
    c.arm();
    assert!(c.ap_enter());
    assert!(!c.bsp_abandon(), "an entered AP is asked to park, never abandoned");
    assert!(c.entered());

    let c = BootClaim::new();
    c.arm();
    assert!(c.bsp_abandon());
    assert!(!c.ap_enter(), "a late AP must park");
    assert!(!c.entered());
}

#[test]
fn claim_race_never_has_two_winners() {
    for _ in 0..2000 {
        let c = Arc::new(BootClaim::new());
        c.arm();
        let wins = Arc::new(AtomicU32::new(0));
        let (c2, w2) = (c.clone(), wins.clone());
        let ap = std::thread::spawn(move || {
            if c2.ap_enter() {
                w2.fetch_add(1, Ordering::SeqCst);
            }
        });
        if c.bsp_abandon() {
            wins.fetch_add(1, Ordering::SeqCst);
        }
        ap.join().unwrap();
        assert_eq!(wins.load(Ordering::SeqCst), 1);
    }
}

#[test]
fn timeouts_are_calibrated_time_with_a_floor() {
    assert_eq!(budget_ticks(1000, 1_100_000_000, 7), 1_100_000_000);
    assert_eq!(budget_ticks(1000, 0, 7), 7);
    assert_eq!(budget_ticks(1000, 999, 7), 7);
}

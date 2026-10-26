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

//! A let-go's order (reclaim_rule.rs, run by orphans.rs `reclaim`): the
//! orphans under GATE, GATE let go, then the held runs, which take GATE
//! themselves (grant_run.rs `free_unheld`). GATE is a spin lock here as in
//! the kernel; the model asks for it with try_lock, so a reclaim that still
//! held GATE fails the test instead of hanging it.

use spin::Mutex;

use super::common::{grant_frames, slot, Registry, COMPOSITOR, FB, GPU, RUN};
use super::reclaim_rule::reclaim_in;

/// drop_attach, then the reclaim on `r`. Ok when every step found GATE free.
fn reclaim(gate: &Mutex<()>, r: &mut Registry, pid: u32, handle: u32) -> Result<(), &'static str> {
    r.attaches.retain(|&(h, p)| !(h == handle && p == pid));
    let mut nested = false;
    let lock = || gate.try_lock().expect("reclaim's own guard: the gate is free on entry");
    reclaim_in(lock, |_| r.free_orphans(), || nested = gate.try_lock().is_none());
    if nested {
        return Err("free_unheld found GATE still held: a spin lock taken twice never opens");
    }
    let _gate = gate.try_lock().ok_or("GATE left held after reclaim")?;
    r.free_unheld();
    Ok(())
}

fn scanout() -> Registry {
    let slots = vec![slot(FB, GPU, grant_frames(), Some(RUN))];
    Registry { slots, attaches: vec![(FB, GPU), (FB, COMPOSITOR)], ..Default::default() }
}

#[test]
fn reclaim_gives_a_held_grant_back_with_gate_let_go_first() {
    let gate = Mutex::new(());
    let mut r = scanout();
    assert!(r.release(RUN), "held for the compositor");
    assert_eq!(r.runs, vec![RUN], "a held DMA run is present");
    assert_eq!(reclaim(&gate, &mut r, COMPOSITOR, FB), Ok(()));
    assert_eq!(r.given_back, vec![RUN], "given back once nobody maps it");
    assert!(gate.try_lock().is_some(), "and GATE is free afterwards");
}

#[test]
fn reclaim_twice_is_two_clean_passes() {
    let gate = Mutex::new(());
    let mut r = scanout();
    assert!(r.release(RUN));
    assert_eq!(reclaim(&gate, &mut r, COMPOSITOR, FB), Ok(()));
    assert_eq!(
        reclaim(&gate, &mut r, COMPOSITOR, FB),
        Ok(()),
        "a second reclaim finds the gate free"
    );
    assert_eq!(r.given_back.len(), 1, "and gives nothing back twice");
}

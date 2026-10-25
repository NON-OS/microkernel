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
//! reclaim with a held DMA grant present: the orphans under GATE, GATE let
//! go, then reclaim_dma, which takes GATE itself. GATE is a spin lock here as
//! in the kernel; the model's reclaim_dma asks with try_lock, so a reclaim
//! that still held GATE fails the test instead of hanging it.

use spin::Mutex;

use super::common::{grant_frames, Registry, COMPOSITOR, FB, GPU, LEN, START};
use super::reclaim_rule::reclaim_in;

/// reclaim on `r`: drop_attach's detach under the gate, then the held
/// grants, which take the gate. Ok when every step found the gate free.
fn reclaim(gate: &Mutex<()>, r: &mut Registry, pid: u32, handle: u32) -> Result<(), &'static str> {
    let mut nested = false;
    let lock = || gate.try_lock().expect("reclaim's own guard: the gate is free on entry");
    reclaim_in(lock, |_| r.attaches.retain(|&(h, p)| !(h == handle && p == pid)), || {
        nested = gate.try_lock().is_none();
    });
    if nested {
        return Err("reclaim_dma found GATE still held: a spin lock taken twice never opens");
    }
    let _gate = gate.try_lock().ok_or("GATE left held after reclaim")?;
    r.detach(pid, handle);
    Ok(())
}

#[test]
fn reclaim_gives_a_held_grant_back_with_gate_let_go_first() {
    let gate = Mutex::new(());
    let mut r = Registry { attaches: vec![(FB, GPU), (FB, COMPOSITOR)], ..Default::default() };
    assert!(r.release(GPU, vec![(FB, GPU, grant_frames())]), "held for the compositor");
    assert_eq!(r.held.len(), 1, "a held DMA region is present");
    assert_eq!(reclaim(&gate, &mut r, COMPOSITOR, FB), Ok(()));
    assert_eq!(r.given_back, vec![(START, LEN)], "given back once nobody maps it");
    assert!(gate.try_lock().is_some(), "and GATE is free afterwards");
}

#[test]
fn reclaim_twice_is_two_clean_passes() {
    let gate = Mutex::new(());
    let mut r = Registry { attaches: vec![(FB, GPU), (FB, COMPOSITOR)], ..Default::default() };
    assert!(r.release(GPU, vec![(FB, GPU, grant_frames())]));
    assert_eq!(reclaim(&gate, &mut r, COMPOSITOR, FB), Ok(()));
    assert_eq!(reclaim(&gate, &mut r, COMPOSITOR, FB), Ok(()), "a second reclaim finds the gate free");
    assert_eq!(r.given_back.len(), 1, "and gives nothing back twice");
}

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

//! The flip that sleeps on the vertical blank: it lands with one park and
//! no spin, falls back to the poll for good on a pipe whose interrupt
//! never comes, gives up at the deadline on a pipe that stopped, and
//! leaves the interrupt switched off after every flip.

use std::cell::RefCell;
use std::rc::Rc;

use super::flip_tests::{two_buffers, BACK, FRONT};
use super::park_rig::Blanks;
use crate::engine::error::Error;
use crate::engine::flip_park::{flip_parked, FRAME_MS};
use crate::engine::gen::{GEN7_5, GEN9};
use crate::engine::irq_regs::{irq_regs, MASTER_INT_CTL, VBLANK};
use crate::engine::park::{Blank, NoPark, Park};
use crate::engine::pipe::Pipe;
use crate::engine::plane_regs as r;
use crate::model::{Gen9, Polls, Shared};

fn rig() -> (Shared, Blanks) {
    let mut m = two_buffers();
    // Only a park lets a frame pass: a poll would never see the flip.
    m.period = 0;
    let model = Rc::new(RefCell::new(m));
    (Shared(model.clone()), Blanks::new(model))
}

fn go(
    m: &mut Shared,
    park: &mut dyn Park,
    blank: &mut Blank,
    surf: u32,
    polls: u32,
) -> Result<(), Error> {
    let irq = (irq_regs(&GEN9, Pipe::A), park, blank);
    flip_parked(m, Pipe::A, surf, &Polls::new(polls), irq)
}

fn quiet(g: &Gen9) -> bool {
    let ier = g.get(0x4_440C) & VBLANK == 0;
    let imr = g.get(0x4_4404) & VBLANK != 0;
    ier && imr && g.get(MASTER_INT_CTL) == 0 && !g.irq_line()
}

#[test]
fn a_flip_lands_on_one_park_with_no_spin() {
    let (mut m, mut park) = rig();
    let mut blank = Blank::Irq;
    // Three expiry checks: a poll would have run out long before the blank.
    go(&mut m, &mut park, &mut blank, BACK, 3).unwrap();
    assert_eq!(m.0.borrow().get(r::plane_surflive(Pipe::A, 1)), BACK);
    assert_eq!((park.waits.as_slice(), park.acks), (&[FRAME_MS][..], 1));
    assert_eq!(blank, Blank::Irq);
    assert!(quiet(&m.0.borrow()));
    go(&mut m, &mut park, &mut blank, FRONT, 3).unwrap();
    assert_eq!((park.waits.len(), park.acks, blank), (2, 2, Blank::Irq));
    assert_eq!(m.0.borrow().get(r::plane_surflive(Pipe::A, 1)), FRONT);
}

#[test]
fn a_stale_event_does_not_count_as_the_blank() {
    let (mut m, mut park) = rig();
    m.0.borrow_mut().set(0x4_4408, VBLANK);
    go(&mut m, &mut park, &mut blank_irq(), BACK, 3).unwrap();
    assert_eq!(m.0.borrow().get(r::plane_surflive(Pipe::A, 1)), BACK);
    assert_eq!(park.waits.len(), 1);
}

fn blank_irq() -> Blank {
    Blank::Irq
}

#[test]
fn an_unrelated_wake_parks_again() {
    let (mut m, mut park) = rig();
    park.early = 2;
    let mut blank = Blank::Irq;
    go(&mut m, &mut park, &mut blank, BACK, 9).unwrap();
    assert_eq!((park.waits.len(), park.acks, blank), (3, 1, Blank::Irq));
}

#[test]
fn an_interrupt_that_never_comes_is_polled_from_then_on() {
    let (mut m, mut park) = rig();
    park.delivers = false;
    let mut blank = Blank::Irq;
    go(&mut m, &mut park, &mut blank, BACK, 9).unwrap();
    assert_eq!(m.0.borrow().get(r::plane_surflive(Pipe::A, 1)), BACK);
    assert_eq!((park.waits.len(), park.acks, blank), (1, 0, Blank::Poll));
    // The next flip arms nothing and parks not at all.
    m.0.borrow_mut().period = 3;
    m.0.borrow_mut().writes.clear();
    go(&mut m, &mut park, &mut blank, FRONT, 50).unwrap();
    assert_eq!(park.waits.len(), 1);
    assert_eq!(m.0.borrow().writes, vec![(r::plane_surf(Pipe::A, 1), FRONT)]);
    assert_eq!(m.0.borrow().get(r::plane_surflive(Pipe::A, 1)), FRONT);
}

#[test]
fn a_stopped_pipe_times_out_and_keeps_its_interrupt() {
    let (mut m, mut park) = rig();
    park.stopped = true;
    let mut blank = Blank::Irq;
    assert_eq!(go(&mut m, &mut park, &mut blank, BACK, 4), Err(Error::Timeout));
    assert_eq!(m.0.borrow().get(r::plane_surflive(Pipe::A, 1)), FRONT);
    // An unplug is not the interrupt failing.
    assert_eq!(blank, Blank::Irq);
    assert!(park.waits.len() >= 2);
    assert!(quiet(&m.0.borrow()));
}

#[test]
fn a_refused_grant_falls_back_to_the_poll() {
    let (mut m, mut park) = rig();
    park.refused = true;
    m.0.borrow_mut().period = 3;
    let mut blank = Blank::Irq;
    go(&mut m, &mut park, &mut blank, BACK, 50).unwrap();
    assert_eq!((park.waits.len(), blank), (1, Blank::Poll));
    assert_eq!(m.0.borrow().get(r::plane_surflive(Pipe::A, 1)), BACK);
    assert!(quiet(&m.0.borrow()));
}

#[test]
fn no_grant_and_haswell_write_only_the_surface() {
    for (gen, bound) in [(&GEN9, false), (&GEN7_5, true)] {
        let (mut m, mut park) = rig();
        m.0.borrow_mut().period = 3;
        m.0.borrow_mut().writes.clear();
        let mut none = NoPark;
        let p: &mut dyn Park = if bound { &mut park } else { &mut none };
        let irq = (irq_regs(gen, Pipe::A), p, &mut Blank::Irq);
        flip_parked(&mut m, Pipe::A, BACK, &Polls::new(50), irq).unwrap();
        assert_eq!(m.0.borrow().writes, vec![(r::plane_surf(Pipe::A, 1), BACK)]);
        assert!(park.waits.is_empty());
    }
}

#[test]
fn a_misaligned_surface_is_refused_before_any_write() {
    let (mut m, mut park) = rig();
    m.0.borrow_mut().writes.clear();
    let r = go(&mut m, &mut park, &mut Blank::Irq, BACK + 0x1000, 9);
    assert_eq!(r, Err(Error::Argument));
    assert!(m.0.borrow().writes.is_empty());
}

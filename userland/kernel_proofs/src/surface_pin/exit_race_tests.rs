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

//! A pin racing its guest's exit. The exit counts the group's surfaces under
//! the pin gate (keep_at_exit), drops its slots (release_owned_by_pid), turns
//! the thread into a zombie, counts and drops again at finalize, then frees
//! the tables but what was kept. A pin and the compositor's attach are put
//! at every point between those steps. With the guest's group marked at the
//! count (pin/exited.rs) no view ever reads a frame the teardown freed;
//! checked by the pid's state alone, as before, one order does.

const PIN: &str = include_str!("../../../../src/kernel_core/surface_registry/pin/pin_guest.rs");
const EXIT: &str = include_str!("../../../../src/kernel_core/surface_registry/pin/exit.rs");
const SURFACES: &str = include_str!("../../../../src/process/exit/surfaces.rs");
const RELEASE: &str = include_str!("../../../../src/process/address_space/lifecycle/release.rs");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Step {
    Count,
    Drop,
    Zombie,
    Free,
    Pin,
    Attach,
}

const EXIT_STEPS: [Step; 6] =
    [Step::Count, Step::Drop, Step::Zombie, Step::Count, Step::Drop, Step::Free];

#[derive(Default)]
struct World {
    marked: bool,
    zombie: bool,
    /// The pinned slot: (frames still listed, attach refused while closing).
    slot: Option<(bool, bool)>,
    right: bool,
    viewed: bool,
    kept: bool,
    freed: bool,
}

impl World {
    /// The compositor's view reads freed frames: the failure.
    fn step(&mut self, s: Step, marks: bool) -> bool {
        match s {
            Step::Count => {
                self.marked |= marks;
                if let Some((frames, closing)) = self.slot.as_mut() {
                    *closing = true;
                    if self.viewed && *frames {
                        self.kept = true;
                    }
                    *frames = false;
                }
            }
            Step::Drop => {
                self.slot = None;
                self.right = false;
            }
            Step::Zombie => self.zombie = true,
            Step::Free => self.freed = !self.kept,
            Step::Pin => {
                if !self.zombie && !(marks && self.marked) && self.slot.is_none() && !self.freed {
                    self.slot = Some((true, false));
                    self.right = true;
                }
            }
            Step::Attach => {
                if matches!(self.slot, Some((true, false))) && self.right {
                    self.viewed = true;
                }
            }
        }
        self.viewed && self.freed
    }
}

/// Whether any order of a pin, then an attach, among the exit's steps lets
/// the view read a freed frame.
fn some_order_reads_freed(marks: bool) -> bool {
    let n = EXIT_STEPS.len();
    for pin_at in 0..=n {
        for attach_at in pin_at..=n {
            let mut w = World::default();
            let mut bad = false;
            for i in 0..=n {
                if i == pin_at {
                    bad |= w.step(Step::Pin, marks);
                }
                if i == attach_at {
                    bad |= w.step(Step::Attach, marks);
                }
                if let Some(&s) = EXIT_STEPS.get(i) {
                    bad |= w.step(s, marks);
                }
            }
            if bad {
                return true;
            }
        }
    }
    false
}

#[test]
fn with_the_group_marked_at_the_count_no_view_reads_a_freed_frame() {
    assert!(!some_order_reads_freed(true));
}

#[test]
fn checked_by_the_pids_state_alone_a_pin_after_the_count_could() {
    assert!(some_order_reads_freed(false), "the model sees the window the mark closes");
}

#[test]
fn the_pin_refuses_a_marked_group_under_the_gate_before_its_slot() {
    let body = PIN.split("pub fn pin_guest").nth(1).expect("the pin");
    let gate = body.find("lock_responsive(&GATE)").expect("the gate");
    let mark = body.find("super::exited::has(group)").expect("the mark asked");
    let slot = body.find("register_surface(group, desc, frames, None)").expect("the slot");
    assert!(gate < mark && mark < slot);
}

#[test]
fn the_exit_marks_a_guests_group_before_it_counts_and_the_reap_clears_it() {
    let body = EXIT.split("pub fn keep_at_exit").nth(1).expect("the exit");
    let gate = body.find("lock_responsive(&GATE)").expect("the gate");
    let mark = body.find("super::exited::mark(group)").expect("the mark");
    let count = body.find("close_owned(group)").expect("the count");
    assert!(gate < mark && mark < count);
    let guest = SURFACES.find("is_foreign(pid)").expect("asked of the thread ending");
    assert!(guest < SURFACES.find("keep_at_exit(group,").expect("the count"));
    let freed = RELEASE.find("cleanup_address_space_keeping(asid, &keep)").expect("the free");
    assert!(freed < RELEASE.find("exited::forget(group)").expect("the clear"));
}

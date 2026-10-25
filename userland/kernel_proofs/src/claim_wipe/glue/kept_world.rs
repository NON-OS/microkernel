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

//! One kept run's entry as the DMA side drives the rule: the registry's
//! let-go finding no entry frees the run through its caller, and a settle
//! that frees drops the entry. The knob `late` puts the entry in only after
//! the first event, as a let-go landing between the registry's hold and the
//! keep would find it; that breaks "no run freed while its device can reach
//! it", so the entry must stand before the registry can let go.

use super::super::kept_rule::{on_let_go, on_settle, Act, Run};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Ev {
    LetGo,
    Settle,
}

/// Frees, settles that kept the run for good, a free before the settle, and
/// an entry left behind for frames already freed.
pub(super) struct Kept {
    pub(super) freed: u32,
    pub(super) kept: u32,
    pub(super) early: bool,
    pub(super) stale: bool,
}

pub(super) fn run(held: bool, frees: bool, order: &[Ev], late: bool) -> Kept {
    let fresh = Run { let_go: !held, settled: false };
    let (mut entry, mut settled) = ((!late).then_some(fresh), false);
    let mut w = Kept { freed: 0, kept: 0, early: false, stale: false };
    let kept = |(r, act): (Run, Act)| (Some(r), act);
    for (i, &ev) in order.iter().enumerate() {
        let (after, act) = match (ev, entry) {
            (Ev::LetGo, None) => (None, Act::Free),
            (Ev::Settle, None) => (None, Act::Wait),
            (Ev::LetGo, Some(r)) => kept(on_let_go(r)),
            (Ev::Settle, Some(r)) => kept(on_settle(r, frees)),
        };
        settled |= ev == Ev::Settle;
        entry = after.filter(|_| act != Act::Free);
        w.freed += u32::from(act == Act::Free);
        w.kept += u32::from(act == Act::Keep);
        w.early |= act == Act::Free && !settled;
        entry = entry.or((late && i == 0).then_some(fresh));
    }
    w.stale = entry.is_some() && w.freed > 0;
    w
}

#[test]
fn the_entry_must_stand_before_the_registry_can_let_go() {
    let order = [Ev::LetGo, Ev::Settle];
    let w = run(true, true, &order, false);
    assert!(w.freed == 1 && !w.early && !w.stale, "in time: freed once, after the wipe");
    let w = run(true, true, &order, true);
    assert!(w.freed == 1 && w.early && w.stale, "late: freed under the device, entry left");
}

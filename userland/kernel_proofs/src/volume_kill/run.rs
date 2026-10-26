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

//! One scenario: H runs an operation and is cut at a request (killed asleep
//! in it, then swept); C runs its whole operation from the shared mount;
//! the machine reboots from the newest header. Then the judge.

use super::disk::{Disk, FIRST_FREE};
use super::exec::Run;
use super::judge::judge;
use super::kinds::{Dir, Fate, Knob, Mount, Op};

/// After the reboot, a pointer to a block past the header's free count.
pub(super) const ABOVE: u8 = 1;
/// After the reboot, a pointer to a block never written.
pub(super) const UNWRITTEN: u8 = 2;
/// While running, a live block allocated again.
pub(super) const REUSE: u8 = 4;
/// C's operation gone after the reboot.
pub(super) const LOST: u8 = 8;

pub(super) fn run<'a>(disk: &'a mut Disk, mount: &'a mut Mount, dir: Dir) -> Run<'a> {
    let (sent, bad, slot, first, rec, after) = (0, 0, [0; 4], 0, 0, 0);
    Run { disk, mount, dir, stop: None, sent, flight: None, bad, slot, first, rec, after }
}

pub(super) fn scenario(dir: Dir, h: Op, c: Op, stop: Option<usize>, fate: Fate, k: Knob) -> u8 {
    let (mut disk, start) = (Disk::new(dir), Mount { generation: 1, free: FIRST_FREE });
    let mut held = start;
    let (flight, bad) = {
        let mut r = run(&mut disk, &mut held, dir);
        r.stop = stop;
        r.op(h);
        (r.flight.take(), r.bad)
    };
    let mut mount = if k == Knob::LocalCopy { start } else { held };
    if fate == Fate::Lands && k != Knob::NoFifo {
        flight.iter().for_each(|w| disk.land(w));
    }
    let mut r = run(&mut disk, &mut mount, dir);
    r.op(c);
    let (slot, c_bad) = (r.slot, r.bad);
    if fate == Fate::Lands && k == Knob::NoFifo {
        flight.iter().for_each(|w| disk.land(w));
    }
    bad | c_bad | judge(&disk, c, slot)
}

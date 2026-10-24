// NONOS Operating System (AGPL-3.0-or-later)
//! One frame as the controller runs it: the frame number on, the periodic
//! list from this frame's HCCA entry (one TD per ED), then the control and
//! bulk lists while their filled bits are set, each ED's queue run until
//! it empties, NAKs or halts. A list found with nothing to run has its
//! filled bit cleared, as the controller does.

use super::td::{run as run_td, Step};
use super::world::World;
use crate::facts::ed::{ED_K, HEAD_H, PTR_MASK};
use crate::facts::regs::{CS_BLF, CS_CLF, CTL_BLE, CTL_CLE, CTL_PLE};

pub fn run(w: &mut World) {
    w.frame = w.frame.wrapping_add(1);
    if w.control & CTL_PLE != 0 && w.hcca != 0 {
        let head = w.rd32(w.hcca + 4 * (w.frame as u32 % 32));
        list(w, head, 1);
    }
    for (enabled, filled, head) in [(CTL_CLE, CS_CLF, w.ctrl_head), (CTL_BLE, CS_BLF, w.bulk_head)]
    {
        if w.control & enabled != 0 && w.status & filled != 0 && !list(w, head, 64) {
            w.status &= !filled;
        }
    }
}

/// Walk a list of EDs; whether any had TDs to run.
fn list(w: &mut World, mut ed: u32, per_ed: usize) -> bool {
    let mut busy = false;
    for _ in 0..64 {
        if ed == 0 {
            return busy;
        }
        busy |= queue(w, ed, per_ed);
        ed = w.rd32(ed + 12) & PTR_MASK;
    }
    panic!("an ED list loops");
}

fn queue(w: &mut World, ed: u32, limit: usize) -> bool {
    if w.rd32(ed) & ED_K != 0 {
        return false;
    }
    let mut any = false;
    for _ in 0..limit {
        let (head, tail) = (w.rd32(ed + 8), w.rd32(ed + 4) & PTR_MASK);
        if head & HEAD_H != 0 || head & PTR_MASK == tail {
            break;
        }
        any = true;
        if run_td(w, ed, head & PTR_MASK) != Step::Retired {
            break;
        }
    }
    any
}

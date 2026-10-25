// NONOS Operating System (AGPL-3.0-or-later)
//! The uncut run, stated apart from the interpreter: a mutex below the
//! sync level of the last one taken cannot be acquired (not even one
//! already held); one held is taken again deeper; releasing one not held
//! faults; \_GL is level 0 and takes the Global Lock, which firmware
//! holding for good turns into running out of time; Notify queues, Sleep
//! and Stall wait. After each operation: notifications, mutexes held,
//! sleeps and stalls so far.

use super::gen::{Op, Scene};
use super::held::{give, take};
use crate::exec::FaultKind;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Snap {
    pub notes: usize,
    pub held: usize,
    pub sleeps: usize,
    pub stalls: usize,
}

pub struct Uncut {
    pub end: Option<FaultKind>,
    /// `snaps[j]`: after the first `j` operations.
    pub snaps: Vec<Snap>,
    pub notes: Vec<u8>,
    /// Mutexes still held when the run ended, for the clean-up to give back.
    pub held_end: usize,
}

pub fn uncut(sc: &Scene) -> Uncut {
    let (mut held, mut depth) = (Vec::<(u8, u8)>::new(), [0u32; 4]);
    let mut snap = Snap::default();
    let mut out = Uncut { end: None, snaps: vec![snap], notes: Vec::new(), held_end: 0 };
    for o in &sc.ops {
        let r = match *o {
            Op::Acquire(n, _) => take(n, sc.levels[n as usize], &mut held, &mut depth),
            Op::Release(n) => give(n, &mut held, &mut depth),
            Op::AcquireGl => take(3, 0, &mut held, &mut depth).and_then(|_| {
                if sc.holds == Some(u32::MAX) && depth[3] == 1 {
                    return Err(FaultKind::Time);
                }
                Ok(())
            }),
            Op::ReleaseGl => give(3, &mut held, &mut depth),
            Op::Notify(v) => {
                out.notes.push(0x80 + v);
                snap.notes += 1;
                Ok(())
            }
            Op::Sleep(_) | Op::Stall(_) => {
                let sleep = matches!(o, Op::Sleep(_));
                (snap.sleeps, snap.stalls) =
                    (snap.sleeps + usize::from(sleep), snap.stalls + usize::from(!sleep));
                Ok(())
            }
        };
        snap.held = held.len();
        out.held_end = held.len();
        if let Err(k) = r {
            out.end = Some(k);
            return out;
        }
        out.snaps.push(snap);
    }
    out
}

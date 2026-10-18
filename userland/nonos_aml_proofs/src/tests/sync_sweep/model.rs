// NONOS Operating System (AGPL-3.0-or-later)
//! The model of the sync operators (ACPI 6.5, 19.6.2, 19.6.117, 19.6.152,
//! 19.6.126, 19.6.119; recalled, unverified), with NONOS's single
//! evaluation: a mutex is taken only at a sync level no lower than the
//! last one held; taken again by its holder, it nests; held by another
//! evaluation, it can never be freed, so a timed Acquire sleeps out its
//! timeout and one for ever (0xFFFF) is a fault. A Wait on an unsignalled
//! event sleeps out its timeout, for ever being a fault. Every sleep is
//! charged to the time budget first; one past it is a fault. Whatever is
//! held when the evaluation ends is released and counted.

use super::gen::{Program, SyncOp};
use crate::exec::FaultKind;

#[derive(Debug, PartialEq, Eq)]
pub struct Outcome {
    pub fault: Option<FaultKind>,
    pub slept: Vec<u64>,
    pub events: [u32; 2],
    pub left: u32,
}

pub fn run(p: &Program) -> Outcome {
    let (mut held, mut events, mut spent) = (Vec::<(usize, u8, u32)>::new(), [0u32; 2], 0u64);
    let mut slept = Vec::new();
    let mut sleep = |ms: u64, spent: &mut u64| {
        let total = spent.saturating_add(ms * 1000);
        (total <= p.time_ms * 1000)
            .then(|| (*spent = total, slept.push(ms)).0)
            .ok_or(FaultKind::Time)
    };
    let fault = p.ops.iter().try_for_each(|o| match *o {
        SyncOp::Acquire(m, t) => {
            let level = p.levels[m];
            if held.last().is_some_and(|h| h.1 > level) {
                return Err(FaultKind::Mutex);
            }
            if let Some(h) = held.iter_mut().find(|h| h.0 == m) {
                h.2 += 1;
            } else if p.foreign[m] && t == u16::MAX {
                return Err(FaultKind::Mutex);
            } else if p.foreign[m] {
                sleep(u64::from(t), &mut spent)?;
            } else {
                held.push((m, level, 1));
            }
            Ok(())
        }
        SyncOp::Release(m) => {
            let i = held.iter().position(|h| h.0 == m).ok_or(FaultKind::Mutex)?;
            held[i].2 -= 1;
            if held[i].2 == 0 {
                held.remove(i);
            }
            Ok(())
        }
        SyncOp::Wait(v, _) if events[v] > 0 => {
            events[v] -= 1;
            Ok(())
        }
        SyncOp::Wait(_, t) if t >= 0xFFFF => Err(FaultKind::Time),
        SyncOp::Wait(_, t) => sleep(t, &mut spent),
        SyncOp::Signal(v) => {
            events[v] = events[v].saturating_add(1);
            Ok(())
        }
        SyncOp::Reset(v) => {
            events[v] = 0;
            Ok(())
        }
    });
    Outcome { fault: fault.err(), slept, events, left: held.len() as u32 }
}

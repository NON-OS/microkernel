// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's call window (its xhci/window.rs), kept per test thread so
//! tests running side by side do not share one, and the time each call to
//! the scripted controller spends under it.

use std::cell::Cell;

use super::device::device;
use crate::clock::{advance, now_ms};
use crate::protocol::{E_IO, E_TIMEDOUT};

/// What a call the device answers costs.
const CALL_MS: u64 = 1;
/// How long driver.xhci0 waits on a transfer the device never completes.
pub const XHCI_GIVES_UP_MS: u64 = 5_000;
const NONE: u64 = u64::MAX;

thread_local! {
    static END_MS: Cell<u64> = const { Cell::new(NONE) };
}

pub struct Window {
    outer: u64,
}

impl Window {
    pub fn open(ms: u64) -> Self {
        let end = now_ms().map_or(0, |t| t.saturating_add(ms).min(NONE - 1));
        Self { outer: END_MS.with(|e| e.replace(end)) }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        END_MS.with(|e| e.set(self.outer));
    }
}

pub fn left_ms(cap: u64) -> u64 {
    let end = END_MS.with(Cell::get);
    if end == NONE {
        return cap;
    }
    now_ms().map_or(0, |now| end.saturating_sub(now)).min(cap)
}

/// A call that takes `ms` of the controller's time: all of it, or what the
/// open window has left, and then it fails `E_TIMEDOUT` as the driver's own
/// call does.
fn spend(ms: u64) -> Result<(), i32> {
    let left = left_ms(ms);
    advance(left);
    if left < ms {
        Err(E_TIMEDOUT)
    } else {
        Ok(())
    }
}

/// What the next call costs, and whether the device answers it.
pub(super) fn cost() -> (u64, bool) {
    device(|d| d.silent_ms.map_or((d.call_ms.max(CALL_MS), true), |ms| (ms, false)))
}

/// One call to a device that answers, unless it has gone silent.
pub fn call() -> Result<(), i32> {
    let (ms, answered) = cost();
    spend(ms).and(if answered { Ok(()) } else { Err(E_IO) })
}

/// A transfer the device never completes, which driver.xhci0 answers
/// `E_IO` once it gives up.
pub fn never_completes() -> Result<usize, i32> {
    spend(XHCI_GIVES_UP_MS).and(Err(E_IO))
}

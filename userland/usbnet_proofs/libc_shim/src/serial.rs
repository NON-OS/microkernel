// NONOS Operating System (AGPL-3.0-or-later)
//! One proof at a time where the core keeps state of its own: the bind
//! budget in its xHCI client is process wide, as it is in the capsule,
//! while the clock here is per thread, so two searches run side by side
//! would each see the other's budget.

use std::sync::{Mutex, MutexGuard};

static SERIAL: Mutex<()> = Mutex::new(());

/// Hold this for the whole of a proof that reaches driver.xhci0. A proof
/// that failed poisons nothing: the next one still runs.
pub fn serial() -> MutexGuard<'static, ()> {
    SERIAL.lock().unwrap_or_else(|e| e.into_inner())
}

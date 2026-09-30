// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{fence, Ordering};
use spin::Mutex;

use crate::process::scheduler::selection::cpu_on_tables;
use crate::process::Pid;

static PENDING: Mutex<Vec<Pid>> = Mutex::new(Vec::new());

pub(super) fn enqueue(pid: Pid) {
    let mut q = PENDING.lock();
    if !q.contains(&pid) {
        q.push(pid);
    }
}

/// Finalize every queued teardown whose tables no CPU can still be using.
///
/// Runs from any CPU's timer trap. With several CPUs, a process queued here
/// can still be on another one: running there, killed from here and not yet
/// switched away from, or leaving it mid-switch. Finalizing then would free
/// the tables that CPU translates through. Such a pid stays queued and is
/// tried again on a later tick. A CPU that waits after its process died has
/// loaded the kernel's tables first, so it does not hold the pid back (see
/// `park`).
pub(crate) fn drain() {
    let ready: Vec<Pid> = match PENDING.try_lock() {
        Some(mut q) => {
            if q.is_empty() {
                return;
            }
            // Pairs with the fence in the switch: see `switch_to_process`.
            fence(Ordering::SeqCst);
            let mut ready = Vec::new();
            q.retain(|&pid| {
                let in_use = cpu_on_tables(pid).is_some();
                if !in_use {
                    ready.push(pid);
                }
                in_use
            });
            ready
        }
        None => return,
    };
    for pid in ready {
        super::finalize::finalize_teardown(pid);
    }
}

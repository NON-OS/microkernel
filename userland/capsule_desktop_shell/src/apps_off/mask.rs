/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

use core::mem::size_of;
use core::sync::atomic::{AtomicU16, Ordering};

use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader};

/* The procstat layout that first carried the app switches. */
const APPS_VERSION: u32 = 4;
const UNREAD: u16 = 1 << 8;

static OFF: AtomicU16 = AtomicU16::new(UNREAD);

/* Whether setup turned off the app that owns `service`. */
pub fn is_off(service: &[u8]) -> bool {
    nonos_policy_proto::apps::is_off(off(), service)
}

/*
 * Read once and kept: setup has ended before the shell starts, and nothing
 * turns an app back on before the next boot. A kernel that would not say is
 * asked again next time, every app counted on meanwhile.
 */
fn off() -> u8 {
    let known = OFF.load(Ordering::Relaxed);
    if known & UNREAD == 0 {
        return known as u8;
    }
    let Some(off) = read() else { return 0 };
    OFF.store(off as u16, Ordering::Relaxed);
    off
}

fn read() -> Option<u8> {
    const LEN: usize = size_of::<ProcStatHeader>() + size_of::<ProcStatEntry>();
    let mut buf = [0u8; LEN];
    if mk_proc_stat(buf.as_mut_ptr(), 1) < 0 {
        return None;
    }
    /* The buffer holds a whole header at its start, read unaligned. */
    let h = unsafe { core::ptr::read_unaligned(buf.as_ptr() as *const ProcStatHeader) };
    (h.version >= APPS_VERSION).then_some(h.apps_off as u8)
}

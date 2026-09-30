/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The apps first-boot setup turned off, as the kernel holds them. The kernel
 * spawns none of them, at boot or for a dock click, so the dock shows each
 * as off and says why a click on one opens nothing.
 */

use core::mem::size_of;
use core::sync::atomic::{AtomicU16, Ordering};

use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader};
use nonos_policy_proto::apps::is_off;

use crate::render::palette;
use crate::state::{Context, NotifyLevel};

/* The procstat layout that first carried the app switches. */
const APPS_VERSION: u32 = 4;
const UNREAD: u16 = 1 << 8;

static OFF: AtomicU16 = AtomicU16::new(UNREAD);

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

/* A dock tile's fill when its app is neither open nor focused. */
pub fn idle_fill(service: &[u8]) -> u32 {
    if is_off(off(), service) {
        palette::TILE_OFF
    } else {
        palette::TILE_FILL
    }
}

/* Say why a dock click opened nothing. */
pub fn toast_failed(ctx: &mut Context, service: &[u8], now: i64) {
    let text: &[u8] = match is_off(off(), service) {
        true => b"turned off at setup",
        false => b"could not open window",
    };
    ctx.toasts.push(text, NotifyLevel::Error, now);
}

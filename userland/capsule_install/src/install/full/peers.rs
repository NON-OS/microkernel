/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The compositor and the input router, waited for rather than asked once:
 * both start before setup, so on an install boot they are there, and the
 * wait bounds only a desktop that never came.
 */

use nonos_app_skeleton::discover::lookup_port;
use nonos_libc::{mk_yield, Deadline};

const PEERS_WAIT_MS: u64 = 60_000;

/* (compositor, input router) ports. */
pub(super) fn wait() -> Result<(u32, u32), &'static str> {
    let until = Deadline::after_ms(PEERS_WAIT_MS);
    loop {
        match (lookup_port(b"compositor"), lookup_port(b"input_router")) {
            (Some(c), Some(r)) => return Ok((c, r)),
            _ if until.expired() => return Err("no compositor or input router"),
            _ => {
                let _ = mk_yield();
            }
        }
    }
}

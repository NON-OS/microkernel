/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * Setup's exit status is the one word the kernel reads from it. Its low byte
 * says what to start once setup has gone, the desktop or the installer, and
 * the byte above it the apps turned off. Nothing is carried above those two.
 */

pub const EXIT_SHIFT: u32 = 8;

/* The status setup exits with: `what` to start, and the apps `off`. */
pub fn exit_status(what: u8, off: u8) -> i32 {
    what as i32 | (off as i32) << EXIT_SHIFT
}

/*
 * What to start and the apps off, or `None` for a status setup does not
 * write: a negative one, or one with bits above the apps byte.
 */
pub fn exit_parts(status: i32) -> Option<(u8, u8)> {
    if !(0..1 << (EXIT_SHIFT + 8)).contains(&status) {
        return None;
    }
    Some(((status & 0xFF) as u8, (status >> EXIT_SHIFT) as u8))
}

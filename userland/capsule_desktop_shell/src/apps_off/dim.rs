/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/* How much of an off app's mark still shows: enough to read, plainly not live. */
const OFF_ALPHA_PCT: u32 = 35;

/* `argb` with its alpha cut to OFF_ALPHA_PCT percent, the colour kept. */
pub const fn dim(argb: u32) -> u32 {
    let alpha = (argb >> 24) * OFF_ALPHA_PCT / 100;
    (alpha << 24) | (argb & 0x00FF_FFFF)
}

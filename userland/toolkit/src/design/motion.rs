/* The NØNOS motion durations, in milliseconds. Motion only explains a change
 * of state: a hover or press answers in QUICK, a menu, toast or popover arrives
 * in SETTLE, and a window or the Launchpad moves in MOVE. Anything leaving takes
 * two thirds of its arrival. With animations off, every one lands at its end at
 * once (animation::enabled). */
pub const QUICK_MS: u32 = 90;
pub const SETTLE_MS: u32 = 180;
pub const MOVE_MS: u32 = 260;

/// How long something takes to leave that arrived in `arrive` ms.
pub const fn leave_ms(arrive: u32) -> u32 {
    arrive * 2 / 3
}

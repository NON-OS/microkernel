// NONOS Operating System (AGPL-3.0-or-later)
//! Which root port's device tree a QH's transactions reach, and whether
//! the QH reaches its device the way it sits: its speed in EPS, the
//! control flag only on a slower control endpoint, the TT of a split
//! transaction naming the nearest high-speed hub and the port toward the
//! device, and a periodic split with completes to collect. A QH that gets
//! any of these wrong sees a transaction error (EHCI 1.0 sections 3.6 and
//! 4.12, as recalled, unverified).

use super::bus::path;
use super::world::{Speed, World};
use crate::facts::port::PORT_PE;
use crate::facts::qh::CH_C;

/// XactErr in a qTD token.
const XACT: u32 = 0x08;

/// The root port index whose tree holds the QH's device, or the halt bits.
pub fn reach(w: &World, qh: u32, periodic: bool) -> Result<usize, u32> {
    let (ch, caps) = (w.rd32(qh + 4), w.rd32(qh + 8));
    let (fa, ep) = ((ch & 0x7F) as u8, (ch >> 8) & 0xF);
    let found = (0..w.devs.len()).find_map(|i| {
        let up = w.ports[i] & PORT_PE != 0 && w.speed[i] == Speed::High;
        let d = w.devs[i].as_ref().filter(|_| up)?;
        path(d, 3, fa).map(|p| (i, p))
    });
    let (i, p) = found.ok_or(XACT)?;
    let eps = (ch >> 12) & 3;
    if eps != [2, 0, 1, 2][p.speed.min(3) as usize] {
        return Err(XACT);
    }
    let control = ch & CH_C != 0;
    if p.speed == 3 {
        return if control { Err(XACT) } else { Ok(i) };
    }
    let tt = (((caps >> 16) & 0x7F) as u8, ((caps >> 23) & 0x7F) as u8);
    let cmask = (caps >> 8) & 0xFF;
    if p.tt != Some(tt) || control != (ep == 0) || (periodic && cmask == 0) {
        return Err(XACT);
    }
    Ok(i)
}

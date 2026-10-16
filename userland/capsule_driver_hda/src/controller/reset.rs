// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! A full controller reset, then the codecs that signalled after it.
//!
//! UEFI firmware brings the controller out of reset itself, enumerates the
//! codecs, clears STATESTS behind it and may leave its rings and streams
//! running, so a controller found out of reset is reset again, as Linux does
//! on every probe (`snd_hdac_bus_reset_link`): engines stopped, CRST taken
//! low and seen low, held at least 100 us for the codec PLLs (HDA 1.0a
//! section 5.5.1.2), released and seen high, then the 521 us a codec may
//! take to request a state change (section 4.3) before STATESTS is read.

pub use super::crst::{enter_reset, leave_reset};
use super::stop::stop_engines;
use super::wait::until;
use crate::clock::pause_ms;
use crate::constants::{GCTL, GCTL_CRST, STATESTS, STATESTS_MASK};
use crate::error::HdaResult;
use crate::regs::Regs;

/// How long CRST and the ring run bits are given to move (Linux: 100 ms).
pub const RESET_MS: u64 = 100;
/// Held in reset, and given after it, above the 100 us and 521 us the
/// specification names (Linux sleeps 0.5 to 1 ms and 1 to 1.2 ms).
pub const HOLD_MS: u64 = 1;
pub const SETTLE_MS: u64 = 2;
/// A link with no codec after the settle is watched this much longer: some
/// codecs signal late, and the wait falls only on a machine that has none.
pub const LATE_CODEC_MS: u64 = 20;

/// Reset the controller and the link and return the codecs that answered,
/// one bit per codec address. An engine that will not stop fails the
/// attempt by name; the teardown's own reset then stops it for the next.
pub fn reset_link(regs: Regs) -> HdaResult<u16> {
    stop_engines(regs)?;
    // SAFETY: GCTL and STATESTS lie in every HDA register window.
    unsafe {
        if regs.r32(GCTL) & GCTL_CRST != 0 {
            regs.w16(STATESTS, STATESTS_MASK);
        }
    }
    enter_reset(regs)?;
    pause_ms(HOLD_MS);
    leave_reset(regs)?;
    pause_ms(SETTLE_MS);
    // SAFETY: STATESTS lies in every HDA register window.
    let read = || unsafe { regs.r16(STATESTS) } & STATESTS_MASK;
    let mut mask = read();
    if mask == 0 {
        until(LATE_CODEC_MS, || {
            mask = read();
            mask != 0
        });
    }
    // SAFETY: as above. The bits latch; cleared, a codec that drops off shows.
    unsafe { regs.w16(STATESTS, STATESTS_MASK) };
    Ok(mask)
}

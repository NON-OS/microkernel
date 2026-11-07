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
//! The two Poisson clocks the sender runs on (Loopix, Piotrowska et al.,
//! USENIX Security 2017). At each tick of the sending clock one packet
//! leaves: the oldest real one queued, or a drop cover when none waits. At
//! each tick of the loop clock a loop cover leaves, routed back here. When a
//! packet leaves depends on the clocks alone, never on whether anyone is
//! using the link. Pure, so the host proofs hold it to the model.

use super::exp_delay::exp_delay_ns;

/// The mean gap of each clock, in nanoseconds.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Profile {
    pub send_mean_ns: u64,
    pub loop_mean_ns: u64,
}

/// The shipped default: about five and a half packets a second in all.
pub const LOW_RATE: Profile = Profile { send_mean_ns: 200_000_000, loop_mean_ns: 2_000_000_000 };

/// Nym's own client rates: about fifty-five packets a second.
pub const NYM_RATE: Profile = Profile { send_mean_ns: 20_000_000, loop_mean_ns: 200_000_000 };

/// The profile a configuration byte names: 0 the low rate, 1 Nym's.
pub fn profile_of(code: u8) -> Option<Profile> {
    match code {
        0 => Some(LOW_RATE),
        1 => Some(NYM_RATE),
        _ => None,
    }
}

/// The code `profile_of` reads back for a profile.
pub fn code_of(profile: Profile) -> u8 {
    u8::from(profile == NYM_RATE)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tick {
    Send,
    Loop,
}

/// A clock this far behind starts again from now rather than firing every
/// tick it missed at once. An exponential gap has no memory, so a fresh draw
/// from now is the same process; the missed ticks are a gap, not a burst.
pub const RESYNC_NS: u64 = 1_000_000_000;

#[derive(Clone, Copy, Debug)]
pub struct Clocks {
    send_at: u64,
    loop_at: u64,
}

impl Clocks {
    /// Both clocks started at `now`, each with its own draw.
    pub fn start(now: u64, profile: Profile, send_random: u64, loop_random: u64) -> Self {
        Self {
            send_at: now.saturating_add(exp_delay_ns(send_random, profile.send_mean_ns).max(1)),
            loop_at: now.saturating_add(exp_delay_ns(loop_random, profile.loop_mean_ns).max(1)),
        }
    }

    /// When the next tick of either clock falls.
    pub fn next_ns(&self) -> u64 {
        self.send_at.min(self.loop_at)
    }

    /// The earliest tick due by `now`, its clock moved on by a draw from
    /// `random`; None when neither is due.
    pub fn take(&mut self, now: u64, profile: Profile, random: u64) -> Option<Tick> {
        let send = self.send_at <= self.loop_at;
        let (at, mean) = if send {
            (self.send_at, profile.send_mean_ns)
        } else {
            (self.loop_at, profile.loop_mean_ns)
        };
        if at > now {
            return None;
        }
        let from = if now - at > RESYNC_NS { now } else { at };
        let next = from.saturating_add(exp_delay_ns(random, mean).max(1));
        if send {
            self.send_at = next;
            Some(Tick::Send)
        } else {
            self.loop_at = next;
            Some(Tick::Loop)
        }
    }
}

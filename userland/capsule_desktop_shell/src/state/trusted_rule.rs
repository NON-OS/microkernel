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

//! When the shell's chrome belongs in the compositor's trusted band, and how
//! it moves there and back. A permission prompt, the system's own prompt and
//! the hidden screen are drawn on the chrome; while one is up the chrome is
//! placed in the band, where no window can cover it, no screenshot holds it,
//! and the router gives the shell every key and click.

use super::attention::Attention;

/// The compositor's trusted band (its `state/trusted_rule.rs`), and the
/// chrome's own band under it (`setup/prime/register.rs`).
pub const TRUSTED_Z: u32 = 0x4000_0000;
pub const CHROME_Z: u32 = 3_000_000;

/// Whether the chrome should be in the trusted band: a prompt or the hidden screen
/// is up. Menus and the Launchpad are the shell's but ask nothing.
pub fn in_band(prompt: bool, attention: Attention) -> bool {
    prompt || attention != Attention::None
}

/// From the band the chrome is in (`up`) to the one it should be in: the
/// band to place it in first and the one to take it from after, so it is
/// never shown in neither. None when it is where it should be.
pub fn moves(up: bool, want: bool) -> Option<(u32, u32)> {
    match (up, want) {
        (false, true) => Some((TRUSTED_Z, CHROME_Z)),
        (true, false) => Some((CHROME_Z, TRUSTED_Z)),
        _ => None,
    }
}

/// What the compositor said to the chrome's removal from the band it left.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Removed {
    /// Taken away.
    Taken,
    /// Refused: the band holds no layer of the shell's (the compositor
    /// answers so for an empty band), so there is nothing to take.
    Refused,
    /// No answer in time, or the call failed: the layer may still be there.
    Unanswered,
}

/// Whether the chrome has left the band it was taken from, after the
/// removal was answered `r`. Counted as moved on no answer, the chrome stayed
/// in the trusted band for good, over every window, and the router gave the
/// shell every key and click until a prompt opened again.
pub fn left(r: Removed) -> bool {
    r != Removed::Unanswered
}

/// Where the chrome is: `up` in the trusted band or under it, and whether
/// the other band may still hold it (`stray`), a removal from there not yet
/// answered.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Band {
    pub up: bool,
    pub stray: bool,
}

impl Band {
    /// Placed in the band `want` names: it is there now, whatever the
    /// removal from the other band then gets. Counted as not moved until
    /// that removal was answered, a prompt answered before the next repaint
    /// left the chrome in the trusted band with `up` false, so nothing ever
    /// took it from there: the router gave the shell every key and click,
    /// and every other present of the chrome was refused, its buffer shown
    /// in the trusted band too, and submitted whole.
    pub fn placed(want: bool) -> Band {
        Band { up: want, stray: true }
    }

    /// A place in the other band that was not answered taken: the chrome is
    /// still in the band it was in, and the other may hold it too, as a
    /// compositor that serves the place after the shell's wait puts it
    /// there. Counted as owed, the next sync places it again or, with the
    /// prompt gone, takes it from there. Counted as nothing, a late place
    /// left the chrome in the trusted band with `up` false and no removal
    /// owed, and the router gave the shell every key and click.
    pub fn place_unsure(self) -> Band {
        Band { stray: true, ..self }
    }

    /// The band to take the chrome from now, if any: the one it is not in.
    pub fn take(self) -> Option<u32> {
        self.stray.then_some(if self.up { CHROME_Z } else { TRUSTED_Z })
    }

    /// The removal from the other band was answered `r`; asked again on the
    /// next sync while it was not.
    pub fn taken(self, r: Removed) -> Band {
        Band { stray: self.stray && !left(r), ..self }
    }
}

/// Whether the shell answers Ctrl+Alt+T: never while the chrome is in the
/// band (`up`), so no Terminal opens over a prompt or the hidden screen.
pub fn opens_terminal(up: bool) -> bool {
    !up
}

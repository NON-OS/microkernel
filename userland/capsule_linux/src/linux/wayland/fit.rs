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

//! Full screen for a guest window, as xdg_toplevel says it. Pure, so the
//! host proofs hold it.
//!
//! A guest window had one size for good: the first buffer's. Nothing told
//! the guest the window could be any other, nothing heard it ask, and the
//! window manager never learned it was meant to fill the display, so the
//! dock stayed over it. Now the guest is told the size and the state in a
//! configure, and the window takes them with the first buffer committed
//! after the guest acks that configure, never before: a buffer drawn for
//! the old size is shown where the window was.
//!
//! Maximised and full screen take the same rect, the green button's: the
//! whole width from the foot of the menubar down to the bottom edge. This
//! desktop has no smaller maximised, and the window manager is told both as
//! full screen, so the dock hides for either.

/// x, y, width, height on screen.
pub type Rect = (u32, u32, u32, u32);

/// xdg_toplevel.state: maximized is 1, fullscreen 2.
pub const STATE_MAXIMIZED: u32 = 1;
pub const STATE_FULLSCREEN: u32 = 2;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Mode {
    #[default]
    Normal,
    Maximized,
    FullScreen,
}

impl Mode {
    /// The states a configure for this mode carries.
    pub fn states(self) -> &'static [u32] {
        match self {
            Mode::Normal => &[],
            Mode::Maximized => &[STATE_MAXIMIZED],
            Mode::FullScreen => &[STATE_FULLSCREEN],
        }
    }

    /// Whether the window fills the display, which the window manager is
    /// told with its full-screen flag.
    pub fn fills(self) -> bool {
        self != Mode::Normal
    }
}

/// One configure sent to the guest. `rect` None asks the guest to pick its
/// own size (a configure of 0 by 0), for a window that filled the display
/// before it was ever shown smaller.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Configure {
    pub serial: u32,
    pub mode: Mode,
    pub rect: Option<Rect>,
}

impl Configure {
    /// The size the configure names, 0 by 0 for the guest's own choice.
    pub fn size(&self) -> (u32, u32) {
        self.rect.map_or((0, 0), |r| (r.2, r.3))
    }
}

/// Where a guest window is on its way between sizes.
#[derive(Default)]
pub struct Fit {
    /// The mode the shown window is in.
    mode: Mode,
    /// The rect the window had before it filled the display.
    saved: Option<Rect>,
    /// Told to the guest and not yet acked.
    sent: Option<Configure>,
    /// Acked, and waiting for the buffer drawn for it.
    acked: Option<Configure>,
}

impl Fit {
    /// The mode the window is on its way to: the last one told, else its own.
    pub fn heading(&self) -> Mode {
        self.sent.or(self.acked).map_or(self.mode, |c| c.mode)
    }

    /// Ask for `want`, with the window at `now` and the display's full-screen
    /// rect `fill`. The configure to send, under `serial`, or None when the
    /// window is already on its way there.
    pub fn ask(
        &mut self,
        want: Mode,
        now: Option<Rect>,
        fill: Rect,
        serial: u32,
    ) -> Option<Configure> {
        let heading = self.heading();
        if want == heading {
            return None;
        }
        // Kept once, as it leaves its own size, and not again on the way back.
        if self.mode == Mode::Normal && heading == Mode::Normal {
            self.saved = now;
        }
        let rect = if want.fills() { Some(fill) } else { self.saved };
        let told = Configure { serial, mode: want, rect };
        self.sent = Some(told);
        Some(told)
    }

    /// The guest acked `serial`. Only the configure last sent is taken; an
    /// ack of one it replaced changes nothing.
    pub fn ack(&mut self, serial: u32) {
        if self.sent.is_some_and(|c| c.serial == serial) {
            self.acked = self.sent.take();
        }
    }

    /// The acked configure the next buffer is drawn for.
    pub fn due(&self) -> Option<Configure> {
        self.acked
    }

    /// That buffer is on screen: the window is in its new mode.
    pub fn shown(&mut self) {
        if let Some(c) = self.acked.take() {
            self.mode = c.mode;
            if c.mode == Mode::Normal {
                self.saved = None;
            }
        }
    }
}

/// Where a `width` by `height` window goes when nothing else places it:
/// centred below the top bar, where a guest's window has always opened.
pub fn centred(width: u32, height: u32) -> (u32, u32) {
    let x = 1920u32.saturating_sub(width) / 2;
    let y = 1080u32.saturating_sub(height).saturating_sub(48) / 2 + 48;
    (x, y)
}

/// The corner a buffer is shown at: the acked configure's rect, else where
/// the window is, else centred.
pub fn origin(
    due: Option<Configure>,
    at: Option<(u32, u32)>,
    width: u32,
    height: u32,
) -> (u32, u32) {
    match due {
        Some(Configure { rect: Some((x, y, _, _)), .. }) => (x, y),
        Some(_) => centred(width, height),
        None => at.unwrap_or_else(|| centred(width, height)),
    }
}

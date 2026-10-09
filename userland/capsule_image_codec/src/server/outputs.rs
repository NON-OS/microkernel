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

//! The decoded images image_codec still holds for its clients.
//!
//! Each decode maps a surface for its pixels, registers it and hands the
//! client a handle to attach. Nothing ever let one go: every image decoded
//! kept its memory, up to 64 MiB, and one of the machine's 256 surface slots
//! for as long as image_codec ran, which is as long as the machine. A few
//! hundred images left no slot for any window.
//!
//! An output is let go (unmapped, which gives the slot back once nobody
//! maps it) when its client asks for another decode, since a client asks
//! again only once it has taken the last; when its client has ended; or
//! HOLD_MS after it was made, for a client that never attaches. At most
//! MAX_HELD are held at once, the oldest let go first, so no client can make
//! image_codec hold more. A client that attached before its output was let
//! go keeps its view: the kernel frees the frames when it lets go too.

pub const MAX_HELD: usize = 4;
pub const HOLD_MS: i64 = 30_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Output {
    pub client: u32,
    pub base: usize,
    pub len: usize,
    pub made_ms: i64,
}

pub struct Outputs {
    held: [Option<Output>; MAX_HELD],
}

impl Outputs {
    pub const fn new() -> Self {
        Self { held: [None; MAX_HELD] }
    }

    /// Before serving `client` at `now_ms`: hand `let_go` each output that
    /// client asked for earlier, each one whose client `alive` says ended,
    /// and each one older than HOLD_MS.
    pub fn sweep(
        &mut self,
        client: u32,
        now_ms: i64,
        alive: impl Fn(u32) -> bool,
        mut let_go: impl FnMut(Output),
    ) {
        for slot in self.held.iter_mut() {
            let Some(out) = *slot else { continue };
            let stale = now_ms.saturating_sub(out.made_ms) >= HOLD_MS;
            if out.client == client || stale || !alive(out.client) {
                *slot = None;
                let_go(out);
            }
        }
    }

    /// Hold `out`. With every place taken the oldest is handed back to be
    /// let go, so `out` always gets a place.
    pub fn hold(&mut self, out: Output) -> Option<Output> {
        if let Some(slot) = self.held.iter_mut().find(|s| s.is_none()) {
            *slot = Some(out);
            return None;
        }
        let oldest = (0..MAX_HELD)
            .min_by_key(|&i| self.held[i].map_or(i64::MIN, |o| o.made_ms))
            .unwrap_or(0);
        self.held[oldest].replace(out)
    }
}

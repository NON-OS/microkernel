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
//! The register words for a checked timing. Every field is the count it
//! names less one, as each field's description in Vol 2c says.

use nonos_edid::Detailed;
use nonos_intel_display_hw::error::Error;

use super::check::{check, Sink};

/// One value per timing register, and the pipe's source size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Words {
    pub htotal: u32,
    pub hblank: u32,
    pub hsync: u32,
    pub vtotal: u32,
    pub vblank: u32,
    pub vsync: u32,
    pub srcsz: u32,
}

fn pair(hi: u32, lo: u32) -> u32 {
    (hi << 16) | lo
}

/// The words for `t` on `sink`, or the limit it breaks.
pub fn words(t: &Detailed, sink: Sink) -> Result<Words, Error> {
    check(t, sink)?;
    let (ha, hb, hf, hs) = (t.h_active, t.h_blank, t.h_front, t.h_sync);
    let (va, vb, vf, vs) = (t.v_active, t.v_blank, t.v_front, t.v_sync);
    // Blank end equals the total and blank start the active (TRANS_HBLANK,
    // p. 1360 and 1361; TRANS_VBLANK, p. 1378), so the two words match.
    let htotal = pair(ha + hb - 1, ha - 1);
    let vtotal = pair(va + vb - 1, va - 1);
    Ok(Words {
        htotal,
        hblank: htotal,
        // Start active + front - 1, end active + front + sync - 1 (p. 1363).
        hsync: pair(ha + hf + hs - 1, ha + hf - 1),
        vtotal,
        vblank: vtotal,
        // The same for the vertical sync (TRANS_VSYNC, p. 1393).
        vsync: pair(va + vf + vs - 1, va + vf - 1),
        // The source is the active size, with no panel fitting (p. 704, 705).
        srcsz: pair(ha - 1, va - 1),
    })
}

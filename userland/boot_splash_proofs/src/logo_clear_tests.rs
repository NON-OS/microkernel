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

//! The firmware's logo stands in for the title only where it leaves the
//! attestation panel and the status line clear.

use crate::layout::{logo_clear, panel, status};
use crate::logo::place::Spot;

#[test]
fn a_centred_firmware_logo_gives_way_to_the_title() {
    // OVMF draws its logo centred; a 200 px square at the middle of 1280x800.
    let centred = Spot { x: 540, y: 300, w: 200, h: 200 };
    assert!(!logo_clear(&centred, 1280, 800), "a logo on the attestation panel");
    let (px, py, pw, _) = panel(1280, 800);
    let above = Spot { x: px as i64, y: 40, w: pw, h: py - 40 };
    assert!(logo_clear(&above, 1280, 800), "a logo wholly above the panel is kept");
    let touching = Spot { x: 0, y: (py - 1) as i64, w: 1280, h: 2 };
    assert!(!logo_clear(&touching, 1280, 800), "one row on the panel is on it");
    let (_, sy, _, _) = status(1280, 800);
    let on_status = Spot { x: 10, y: sy as i64, w: 20, h: 1 };
    assert!(!logo_clear(&on_status, 1280, 800), "a logo on the status line");
}

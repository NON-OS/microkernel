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

//! The ring arithmetic and the mode choice.

use crate::fifo::space::free_bytes;
use crate::screen::{choose, Limits};

#[test]
fn free_space_holds_a_word_back_and_refuses_wild_indices() {
    assert_eq!(free_bytes(4096, 4096, 4096, 65536), Some(61436), "empty ring");
    assert_eq!(free_bytes(4100, 4096, 4096, 65536), Some(61432));
    assert_eq!(free_bytes(4092 + 4, 4100, 4096, 65536), Some(0), "full: next just behind stop");
    assert_eq!(free_bytes(8000, 60000, 4096, 65536), Some(51996), "wrapped");
    assert_eq!(free_bytes(65536, 4096, 4096, 65536), None, "next at max");
    assert_eq!(free_bytes(4098, 4096, 4096, 65536), None, "unaligned");
    assert_eq!(free_bytes(4096, 100, 4096, 65536), None, "stop below min");
}

#[test]
fn the_firmware_mode_wins_when_it_fits_and_the_default_otherwise() {
    let lim = Limits { max: (1920, 1200), vram: 16 << 20 };
    assert_eq!(choose(Some((1600, 900)), lim), Some((1600, 900)));
    assert_eq!(choose(Some((2560, 1440)), lim), Some((1024, 768)), "past the maximum");
    assert_eq!(choose(Some((0, 900)), lim), Some((1024, 768)));
    assert_eq!(
        choose(None, Limits { max: (1920, 1200), vram: 1 << 20 }),
        None,
        "no mode fits 1 MiB"
    );
    assert_eq!(choose(None, Limits { max: (700, 500), vram: 16 << 20 }), Some((640, 480)));
}

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
//! Video memory at start (G-72): whatever a dead driver left in it, every
//! byte of it, not just the mode's, is zero before anything is shown.

use crate::dispi::DispiMode;
use crate::edid::Panel;
use crate::model::Adapter;

#[test]
fn a_dead_driver_s_video_memory_is_zeroed_to_its_last_byte() {
    let mut a = Adapter::new();
    let len = a.vram.len();
    a.vram.iter_mut().enumerate().for_each(|(i, b)| *b = i as u8 | 1);
    let b = a.bga(Panel::default(), DispiMode { width: 640, height: 480, stride: 2560 });
    b.wipe();
    assert_eq!(a.vram.len(), len);
    assert!(a.vram.iter().all(|&b| b == 0), "every byte, past the mode's too");
}

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

//! The start-up mode held to the video memory and to the BAR0 mapping it
//! must fit, to the byte: the firmware's kept only while it fits both, and
//! the driver's own refused by name before a register is written.

use super::choose_tests::{ovmf, OVMF};
use crate::constants::DISPI_INDEX_BANK;
use crate::dispi::DispiMode;
use crate::edid::Panel;
use crate::error::BgaError;
use crate::model::Adapter;
use crate::setup::mode::starting;

const MIB: u64 = 1 << 20;
const FALLBACK: DispiMode = DispiMode { width: 1024, height: 768, stride: 1024 * 4 };

#[test]
fn video_memory_and_the_bar0_mapping_each_bound_both_modes_to_the_byte() {
    // 1280x800x4 is 4,096,000 bytes; 1024x768x4 is 3 MiB.
    let kept = |room| starting(ovmf((DISPI_INDEX_BANK, 0)).regs(), &Panel::default(), room);
    let (fw, all) = (4_096_000, 16 * MIB);
    assert_eq!((kept((fw, all)), kept((all, fw))), (Ok(OVMF), Ok(OVMF)));
    assert_eq!((kept((fw - 1, all)), kept((all, fw - 1))), (Ok(FALLBACK), Ok(FALLBACK)));
    let off = |room| starting(Adapter::new().regs(), &Panel::default(), room);
    assert_eq!(off((3 * MIB, all)), Ok(FALLBACK));
    assert_eq!(off((3 * MIB - 1, all)), Err(BgaError::VideoMemoryTooSmall));
    assert_eq!(off((64 * MIB, 3 * MIB)), Ok(FALLBACK));
    assert_eq!(off((64 * MIB, 3 * MIB - 1)), Err(BgaError::ModeTooLarge));
}

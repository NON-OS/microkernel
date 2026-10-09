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

//! Pad drive strength at 3.3 V (rts5227_fill_driving): driver type B, the
//! default sd30_drive_sel_3v3, is row 3 of Linux's driving_3v3 table.

use crate::regs::card::{SD30_CLK_DRIVE_SEL, SD30_CMD_DRIVE_SEL, SD30_DAT_DRIVE_SEL};
use crate::wire::CmdBuf;

const DRIVING_3V3_TYPE_B: [u8; 3] = [0x96, 0x96, 0x96];

pub fn fill_driving_3v3(buf: &mut CmdBuf) {
    buf.write(SD30_CLK_DRIVE_SEL, 0xFF, DRIVING_3V3_TYPE_B[0]);
    buf.write(SD30_CMD_DRIVE_SEL, 0xFF, DRIVING_3V3_TYPE_B[1]);
    buf.write(SD30_DAT_DRIVE_SEL, 0xFF, DRIVING_3V3_TYPE_B[2]);
}

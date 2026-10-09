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

//! The SD commands the bring-up and the reads send (SD Physical Layer
//! 4.7.4), each with its argument and the response it expects.

use super::rsp::Rsp;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Command {
    pub index: u8,
    pub arg: u32,
    pub rsp: Rsp,
}

const fn cmd(index: u8, arg: u32, rsp: Rsp) -> Command {
    Command { index, arg, rsp }
}

/// The host's voltage window, 3.2 to 3.4 V (OCR bits 20 and 21), as
/// realtek_init_host's ocr_avail; HCS asks for a high capacity card.
const VDD_32_34: u32 = 0x0030_0000;
const HCS: u32 = 1 << 30;

impl Command {
    pub const GO_IDLE: Command = cmd(0, 0, Rsp::None);
    pub const ALL_SEND_CID: Command = cmd(2, 0, Rsp::R2);
    pub const SEND_RELATIVE_ADDR: Command = cmd(3, 0, Rsp::R1);
    pub const SEND_IF_COND: Command = cmd(8, super::ocr::IF_COND_ARG, Rsp::R1);
    pub const STOP_TRANSMISSION: Command = cmd(12, 0, Rsp::R1b);
    pub const SET_BLOCKLEN_512: Command = cmd(16, 512, Rsp::R1);

    pub const fn app_cmd(rca: u16) -> Command {
        cmd(55, (rca as u32) << 16, Rsp::R1)
    }
    pub const fn sd_send_op_cond(high_capacity: bool) -> Command {
        cmd(41, VDD_32_34 | if high_capacity { HCS } else { 0 }, Rsp::R3)
    }
    pub const fn send_csd(rca: u16) -> Command {
        cmd(9, (rca as u32) << 16, Rsp::R2)
    }
    pub const fn select_card(rca: u16) -> Command {
        cmd(7, (rca as u32) << 16, Rsp::R1b)
    }
    /// ACMD6 with 10b: a 4-bit bus.
    pub const SET_BUS_WIDTH_4: Command = cmd(6, 2, Rsp::R1);
    pub const fn read_single(addr: u32) -> Command {
        cmd(17, addr, Rsp::R1)
    }
    pub const fn read_multiple(addr: u32) -> Command {
        cmd(18, addr, Rsp::R1)
    }
}

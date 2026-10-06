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

//! Routing the card engine to the SD slot, written before every request as
//! sd_request does (CARD_SELECT, CARD_SHARE_MODE).

use crate::regs::card::{
    CARD_SELECT, CARD_SHARE_48_SD, CARD_SHARE_MASK, CARD_SHARE_MODE, SD_MOD_SEL,
};
use crate::wire::CmdBuf;

pub fn select_sd(buf: &mut CmdBuf) {
    buf.write(CARD_SELECT, 0x07, SD_MOD_SEL);
    buf.write(CARD_SHARE_MODE, CARD_SHARE_MASK, CARD_SHARE_48_SD);
}

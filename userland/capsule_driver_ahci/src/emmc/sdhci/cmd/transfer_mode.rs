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

//! The Transfer Mode register word.

use super::super::regs::{TM_AUTO_CMD12, TM_BLOCK_COUNT, TM_DMA, TM_MULTI, TM_READ};

/// The Transfer Mode register for an ADMA2 data command. Block Count Enable
/// is set for every transfer, single block included (Linux sets it unless a
/// host quirk forbids it). A multi-block transfer either was preceded by
/// CMD23 (the card stops on its own count) or has the host send CMD12 after
/// it (`auto12`). Auto CMD12 on a single block is never asked for.
pub const fn transfer_mode(read: bool, multi: bool, auto12: bool) -> u16 {
    let mut m = TM_DMA | TM_BLOCK_COUNT;
    if multi {
        m |= TM_MULTI;
        if auto12 {
            m |= TM_AUTO_CMD12;
        }
    }
    if read {
        m |= TM_READ;
    }
    m
}

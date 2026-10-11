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

//! A model SD memory card, written from the SD Physical Layer Simplified
//! Specification 9.10 and not from the driver: its states, OCR and power
//! up, CMD8, the APP_CMD prefix, its own address, CID, CSD, bus width and
//! storage. The SDHCI model drives it.

use std::collections::HashMap;

use super::super::card::{Out, Phase, Reply, St};
use super::SdCfg;

/// R1 bits the model sets (4.10.1).
pub const READY_FOR_DATA: u32 = 1 << 8;
pub const APP_CMD: u32 = 1 << 5;
pub const BLOCK_LEN_ERROR: u32 = 1 << 29;
pub const OUT_OF_RANGE: u32 = 1 << 31;
pub const ADDRESS_ERROR: u32 = 1 << 30;

pub struct SdCard {
    pub cfg: SdCfg,
    pub state: St,
    /// The last command was an accepted CMD55.
    pub app: bool,
    pub polls: u32,
    pub width: u8,
    pub store: HashMap<u64, [u8; 512]>,
    pub busy_until: u64,
    pub acmd41_args: Vec<u32>,
    pub writes: u32,
}

pub(super) fn out(reply: Reply) -> Out {
    Out { reply, phase: Phase::None, busy_ms: 0 }
}

impl SdCard {
    pub fn new(cfg: SdCfg) -> Self {
        Self {
            cfg,
            state: St::Off,
            app: false,
            polls: 0,
            width: 1,
            store: HashMap::new(),
            busy_until: 0,
            acmd41_args: Vec::new(),
            writes: 0,
        }
    }

    pub fn power(&mut self, on: bool) {
        if on && self.state == St::Off {
            (self.state, self.polls, self.width, self.app) = (St::Idle, 0, 1, false);
        } else if !on {
            self.state = St::Off;
        }
    }
}

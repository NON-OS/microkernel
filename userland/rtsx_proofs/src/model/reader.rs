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

//! The reader's state and the knobs a test turns, one reader per thread.

use std::cell::RefCell;
use std::collections::HashMap;

use super::card::Card;
use super::knobs::{Knobs, Phase};
use super::run::Run;
use crate::regs::card::{CARD_PWR_CTL, SD_POWER_OFF};

#[derive(Default)]
pub struct Reader {
    pub knobs: Knobs,
    pub regs: HashMap<u16, u8>,
    pub hcbar: u32,
    pub hdbar: u32,
    pub bier: u32,
    pub haimr: u32,
    /// BIPR's latched bits; SD_EXIST and SD_WRITE_PROTECT follow the slot.
    pub pending: u32,
    pub slot: Option<Card>,
    /// A command buffer taken and not finished: stalled, stopping, or at
    /// its check on a data phase.
    pub run: Option<Run>,
    pub data: Option<Phase>,
    /// The engines a stop was written to since they were last started.
    pub cmd_stopped: bool,
    pub dma_stopped: bool,
    /// The SD command indices sent, in order, whether a card took them.
    pub sent: Vec<u8>,
    /// Internal registers reached through HAIMR, read or written.
    pub accesses: u32,
}

thread_local! {
    static READER: RefCell<Reader> = RefCell::new(Reader::new());
}

/// The reader on this thread.
pub fn with<R>(f: impl FnOnce(&mut Reader) -> R) -> R {
    READER.with(|r| f(&mut r.borrow_mut()))
}

impl Reader {
    fn new() -> Self {
        let mut r = Self::default();
        r.regs.insert(CARD_PWR_CTL, SD_POWER_OFF);
        r
    }
}

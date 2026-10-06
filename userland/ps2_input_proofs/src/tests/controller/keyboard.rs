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

//! A keyboard: acknowledges what it is sent, some status reads later.

use crate::constants::MOUSE_ACK;

const KBD_RESET: u8 = 0xFF;
const SELF_TEST_PASSED: u8 = 0xAA;

pub struct Keyboard {
    /// Status reads between a command and its acknowledgement; `None` is a
    /// keyboard that never answers.
    pub latency: Option<u32>,
    /// Status reads between the reset acknowledgement and the self-test
    /// result. Real keyboards take hundreds of milliseconds.
    pub bat: u32,
    /// Ignores everything until it is reset, as a keyboard the EC left
    /// wedged does.
    pub wedged: bool,
    /// A key held down: its make code repeats into the output buffer ahead
    /// of every reply the mouse sends while the keyboard is clocked.
    pub held: Option<u8>,
    /// An EC-emulated controller that tags its own configuration byte as aux
    /// data once the aux clock runs, so a reply filter on AUXDATA misses it.
    pub ctr_aux_tag: bool,
}

impl Keyboard {
    pub const PROMPT: Self =
        Self { latency: Some(0), bat: 2, wedged: false, held: None, ctr_aux_tag: false };
    pub const SLOW: Self = Self { latency: Some(3), ..Self::PROMPT };
    pub const DEAD: Self = Self { latency: None, ..Self::PROMPT };
    /// Wedged until reset, then a self test as long as a real keyboard's.
    pub const WEDGED: Self = Self { latency: Some(1), bat: 400, wedged: true, ..Self::PROMPT };

    /// The bytes a command draws, each with the delay before it appears.
    pub fn command(&mut self, value: u8) -> Vec<(u32, u8)> {
        let Some(latency) = self.latency else {
            return Vec::new();
        };
        match value {
            KBD_RESET => {
                self.wedged = false;
                vec![(latency, MOUSE_ACK), (latency + self.bat, SELF_TEST_PASSED)]
            }
            _ if self.wedged => Vec::new(),
            _ => vec![(latency, MOUSE_ACK)],
        }
    }
}

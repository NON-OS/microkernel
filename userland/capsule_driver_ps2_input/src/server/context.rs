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
use crate::mouse::{MouseParser, MouseRing};
use crate::poll::{Counters, Drainer};
use crate::server::reinits::Reinits;
use crate::setup::Driver;

pub struct Context {
    pub driver: Driver,
    pub counters: Counters,
    pub drainer: Drainer,
    pub mouse: MouseParser,
    pub mouse_ring: MouseRing,
    pub last_kbd_seq: u64,
    pub last_aux_seq: u64,
    /// The lock LEDs as last sent to the keyboard; None before the first.
    pub leds_sent: Option<u8>,
    /// When a mouse that reset was last brought up again.
    pub reinits: Reinits,
}

impl Context {
    pub fn new(driver: Driver) -> Self {
        let mouse = MouseParser::new(driver.mouse_wheel);
        Self {
            driver,
            counters: Counters::default(),
            drainer: Drainer::new(),
            mouse,
            mouse_ring: MouseRing::new(),
            last_kbd_seq: 0,
            last_aux_seq: 0,
            leds_sent: None,
            reinits: Reinits::default(),
        }
    }
}

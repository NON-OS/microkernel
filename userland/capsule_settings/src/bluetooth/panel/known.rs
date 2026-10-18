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

//! What Settings knows of driver.bt_usb0, as the service last answered.

use nonos_bt_proto::{Asked, Bond, Health, Level, Link, BONDS_MAX, LEVELS_MAX};

use super::digits::Digits;
use super::reach::Reach;

#[derive(Default)]
pub struct Panel {
    /// The service's port, as the registry last named it.
    pub port: Option<u32>,
    pub reach: Reach,
    pub health: Option<Health>,
    pub link: Option<Link>,
    pub bonds: [Bond; BONDS_MAX],
    pub bond_count: usize,
    pub levels: [Level; LEVELS_MAX],
    pub level_count: usize,
    /// What pairing asks of the user now.
    pub asked: Asked,
    /// The digits typed so far, when pairing asks for the device's.
    pub digits: Digits,
    /// The highlighted device in the list.
    pub cursor: usize,
    /// What the last thing asked of the service did, for the page to say.
    pub notice: Option<&'static str>,
    /// When this window's PAIR was taken, on the uptime clock, so the page
    /// can count the window down. Another window's PAIR leaves it unset.
    pub window_from_ms: Option<i64>,
    /// When BONDS and BATTERY were last asked.
    pub listed_ms: Option<i64>,
    pub request_id: u32,
}

impl Panel {
    pub const fn new() -> Self {
        Self {
            port: None,
            reach: Reach::Unasked,
            health: None,
            link: None,
            bonds: [Bond { kind: 0, addr: [0; 6], authenticated: false, linked: false }; BONDS_MAX],
            bond_count: 0,
            levels: [Level { place: 0, instance: 0, percent: 0 }; LEVELS_MAX],
            level_count: 0,
            asked: Asked::None,
            digits: Digits::new(),
            cursor: 0,
            notice: None,
            window_from_ms: None,
            listed_ms: None,
            request_id: 0,
        }
    }

    pub fn has(&self, flag: u8) -> bool {
        self.link.is_some_and(|l| l.has(flag))
    }
}

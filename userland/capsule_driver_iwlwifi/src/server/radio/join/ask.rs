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
//! What the radio says about joining: whether it can, and the association.

use crate::server::join_wire::LinkInfo;
use nonos_wifi_core::link_history::LinkHistory;

use super::super::{Radio, State};

impl Radio {
    /// The radio runs joins: it is up, drew a station address and speaks
    /// every join command's layout.
    pub fn can_join(&self) -> bool {
        matches!(&self.state, State::Up(up) if up.addr.is_some() && up.join_api)
    }

    /// The port is open.
    pub fn joined(&self) -> bool {
        self.join.joined.as_ref().is_some_and(|j| j.link.station.is_associated())
    }
    /// How the link last ended and the last join came out.
    pub fn history(&self) -> LinkHistory {
        self.join.history
    }

    /// A join finished with `code`; or the person asked to leave.
    pub fn note_join(&mut self, code: i32) {
        self.join.history.joined(code);
    }

    pub fn note_left(&mut self) {
        self.join.history.left();
    }

    /// The association, for the link reply.
    pub fn link_info(&self) -> Option<LinkInfo<'_>> {
        let j = self.join.joined.as_ref().filter(|j| j.link.station.is_associated())?;
        Some(LinkInfo {
            bssid: j.bss.target.bssid,
            ssid: &self.join.ssid[..self.join.ssid_len],
            akm: j.akm.suite_type(),
        })
    }
}

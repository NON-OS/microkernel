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

//! What a PHY look concluded: up or down, and the speed and duplex the PHY
//! reported (0 Mb/s while unknown).

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LinkState {
    pub up: bool,
    pub mbps: u16,
    pub full: bool,
}

impl LinkState {
    pub const DOWN: LinkState = LinkState { up: false, mbps: 0, full: false };
}

/// Whether `now` is worth a line after `before`. Speed and duplex mean
/// nothing while the link is down, so two downs are the same state.
pub fn changed(before: Option<LinkState>, now: LinkState) -> bool {
    match before {
        None => true,
        Some(b) if !b.up && !now.up => false,
        Some(b) => b != now,
    }
}

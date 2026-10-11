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
//! The answers a connect gives without a join: a hidden network, which this
//! driver cannot find, and a network the hunt did not hear.

use crate::server::join_wire::{ConnectResult, CODE_NOT_FOUND};

use super::super::say::Line;

/// A hidden network: its beacons carry no name, and this driver sends no probe.
pub(super) fn hidden() -> ConnectResult {
    Line::new().text(b"join: a hidden network is not found by listening").send();
    ConnectResult::code(CODE_NOT_FOUND)
}
/// What a connect answers when the hunt heard `heard` beacons, none the network's.
pub(super) fn not_heard(heard: u32) -> ConnectResult {
    Line::new().text(b"join: network not heard among ").num(heard).text(b" beacons").send();
    let mut r = ConnectResult::code(CODE_NOT_FOUND);
    r.progress.recv = heard;
    r
}

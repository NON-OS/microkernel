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

//! The flow control constants, and when an acknowledgement falls due.

/*
 * or.h: CIRCWINDOW_START, CIRCWINDOW_INCREMENT, STREAMWINDOW_START,
 * STREAMWINDOW_INCREMENT. Unchanged in the fork.
 */

pub const CIRCUIT_START: i32 = 1000;
pub const CIRCUIT_INCREMENT: i32 = 100;
pub const STREAM_START: i32 = 500;
pub const STREAM_INCREMENT: i32 = 50;

/// Whether enough cells have arrived on the circuit to owe the far end a
/// SENDME.
pub fn circuit_sendme_due(delivered_since: i32) -> bool {
    delivered_since >= CIRCUIT_INCREMENT
}

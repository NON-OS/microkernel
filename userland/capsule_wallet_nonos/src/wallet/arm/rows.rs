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

//! The rows whose presses move value or sign, by the numbers their screens
//! put them under, so the guard (moves.rs) names the same rows they do.

/// The meter screen's picked channel: top it up, or close it.
pub const ROW_TOPUP: u8 = 40;
pub const ROW_CLOSE: u8 = 41;
/// The meter screen's claim of what the Shield pool credited it.
pub const ROW_CLAIM: u8 = 42;
/// The publish screen's binding, signed once the binding is confirmed.
pub const ROW_BINDING: u8 = 1;
/// Release i's attest is row ROW_ATTEST + i, the releases capped at 16.
pub const ROW_ATTEST: u8 = 2;
/// The Bond's rows from ROW_INTENT, release i's publish, withdraw and
/// anchors at ROW_RELEASE + 3i: each only asks for a quote, and the
/// review's Sign is what signs, so the guard does not take them.
pub const ROW_INTENT: u8 = 20;
pub const ROW_RELEASE: u8 = 30;

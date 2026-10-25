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

//! Where a feature report is read and written: the logic that decides what
//! to write is the transport's to share, HID over I2C or over SPI alike.

/// The longest feature report handled, report id byte included.
pub const REPORT_MAX: usize = 62;

pub trait FeaturePort {
    /// Feature report `id` read into `report`, id byte first; its length,
    /// 0 when the device refused or answered with one that does not fit.
    fn get(&self, id: u8, report: &mut [u8; REPORT_MAX]) -> usize;
    /// `report`, id byte first, written; whether the device took it.
    fn set(&self, report: &[u8]) -> bool;
    /// Whether report `id` has anywhere to be read and written: false where
    /// the transport names no register for it, which no retry changes.
    fn reaches(&self, _id: u8) -> bool {
        true
    }
}

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

//! The bounds the host keeps to, and what it learns of the peripheral.

/// HID service instances kept; a combined keyboard and mouse has two.
pub const MAX_SERVICES: usize = 4;
/// Characteristic declarations kept for one service.
pub const MAX_DECLS: usize = 64;
/// ATT requests in one discovery: far more than a HID device needs, and a
/// bound on a peripheral that answers forever.
pub const MAX_REQUESTS: u32 = 512;
/// The ATT transaction timeout (Vol 3, Part F, 3.3.3): a request with no
/// response in 30 s ends the bearer.
pub const TIMEOUT_MS: u64 = 30_000;

/// One characteristic declaration (Vol 3, Part G, 3.3.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Decl {
    pub handle: u16,
    pub props: u8,
    pub value: u16,
    /// `None` for a 128-bit UUID outside the base range.
    pub uuid: Option<u16>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Keyboard,
    Mouse,
}

/// A boot input report: its value handle, the last handle of its
/// characteristic (where its descriptors end), and its CCCD once found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Source {
    pub kind: Kind,
    pub value: u16,
    pub end: u16,
    pub cccd: Option<u16>,
}

/// A Battery Level read or notified: which Battery Service instance, in the
/// order the peripheral's database holds them, and the charge left in
/// percent (BAS 1.0, 3.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Battery {
    pub instance: u8,
    pub percent: u8,
}

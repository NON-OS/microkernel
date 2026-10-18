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

//! The tool catalogue: stable ids, human names and the NOX price of one use.
//!
//! Both the broker and the tool link this, so neither can drift from the
//! other on what a tool is called or what it costs. Who is paid is not here:
//! the payee comes from the signed listing, never from a constant. Prices are
//! in NOX base units (wei, 1e18 per NOX), matching the amount the wallet signs
//! into a `transfer`.

/// Stable identifiers for the priced tools. The numeric value is what goes on
/// the wire in an entitlement's `tool_id`; never renumber a shipped one.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ToolId {
    /// TCP connect-scan network reconnaissance.
    Recon = 1,
}

impl ToolId {
    /// The wire value.
    pub const fn id(self) -> u32 {
        self as u32
    }

    /// Recover a tool from its wire value.
    pub const fn from_id(id: u32) -> Option<ToolId> {
        match id {
            1 => Some(ToolId::Recon),
            _ => None,
        }
    }
}

/// NOX base units for one use of a tool. `None` for an unknown id, so an
/// unpriced tool can never be sold by accident.
pub const fn price_of(tool_id: u32) -> Option<u128> {
    match ToolId::from_id(tool_id) {
        // 0.25 NOX per scan.
        Some(ToolId::Recon) => Some(250_000_000_000_000_000),
        None => None,
    }
}

/// Display name for a tool id, or `None` if it is not one we sell.
pub const fn tool_name(tool_id: u32) -> Option<&'static str> {
    match ToolId::from_id(tool_id) {
        Some(ToolId::Recon) => Some("recon"),
        None => None,
    }
}

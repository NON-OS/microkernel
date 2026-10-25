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

//! The settings screen's words: its lead and the rows of the key list.

pub(super) const LEAD: &str = "One key holds an account on Ethereum mainnet and on Sepolia. \
     Sepolia is the test network, where the shield pool runs and nothing has value.";

/// The rows of the key list, in press order.
pub const ROWS: [&str; 6] = [
    "Show the private key",
    "Restore from recovery words",
    "Import a private key",
    "Lock the screen",
    "The meter: its budget and channels",
    "Publish a listing: sign its binding and attests",
];

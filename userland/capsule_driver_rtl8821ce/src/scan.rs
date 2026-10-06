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

//! The networks a scan finds. The channel hop and the ring reads live in the
//! serving stage (they need the radio); the collection they feed, its aging
//! and its encoding for the settings panel, and the beacon security flags are
//! shared with the Intel driver in `nonos_wifi_core::scan_list` and checked
//! on the host in `rtl8821ce_proofs`. Signal strength stays at zero here until
//! the per-frame PHY status is decoded.

pub use nonos_wifi_core::scan_list::{beacon_flags, ScanResults};

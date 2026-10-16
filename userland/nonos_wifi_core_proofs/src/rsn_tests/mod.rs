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

//! The RSN negotiation: an access point's RSNE is parsed field by field with
//! its defaults, a malformed one is refused, SAE is chosen whenever offered,
//! a network saved as WPA3 is never joined with PSK, a TKIP group cipher is
//! refused, and the station's RSNE for WPA2-PSK is exactly `wpa::RSN_IE`. The
//! key data walk finds the RSNE, RSNXE, GTK and IGTK and stops at padding.

mod choose;
mod parse;
mod station;

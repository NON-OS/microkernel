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

//! The control family's join messages as this driver reads and answers them;
//! the client side is `nonos_wifi_client::join_wire`, and the RTL8821CE
//! driver answers the same bytes.
//!
//! A connect body is `[ssid_len][ssid][pass_len][pass]` then an optional
//! flags octet (bit 0: saved as WPA3, join with SAE or not at all; bit 1:
//! hidden). Every length is checked against the body, and an empty or
//! overlong SSID is refused. This driver hunts by listening only, so a hidden
//! network is not found.
//!
//! The connect reply is the status code, the exchange's counts (sent,
//! received, data, EAPOL, the first unanswered data frame's frame control
//! and length, deauthentications, frames to this station), the MLME state,
//! the AKM that ran and the access point's own status or reason code. Codes
//! -1 to -12 mean what they mean for the RTL8821CE (`join_text`); this
//! driver answers -1 too when the firmware failed one of the join's
//! commands. The link reply is `[associated][6 zero bytes][ssid_len][ssid][akm]`.

mod codes;
mod connect;
mod link;
mod request;
mod why;

pub use codes::*;
pub use connect::{encode_connect, ConnectResult};
pub use link::*;
pub use request::*;
pub use why::{end_code, failure_code};

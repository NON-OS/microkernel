/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

//! The join and link messages between the client and the drivers, as pure
//! encoding and parsing over untrusted bytes (checked in wifi_panel_proofs).
//!
//! A join body is `[ssid_len][ssid][pass_len][pass][flags]`. The flags octet
//! carries what the saved list knows about the network: bit 0, it was saved
//! as WPA3, so the driver joins it with SAE or not at all (never WPA2, which
//! an attacker offering a WPA2-only copy of the network would otherwise get);
//! bit 1, it is hidden, so the driver may name it in a probe request. A driver
//! that predates the octet ignores it.
//!
//! The connect reply body is `[status i32][sent u32][recv u32][data u32]
//! [eapol u32][probe u32][deauth u32][to_us u32][state u8]`, then `[akm u8]
//! [ap_code u16]` from a driver that runs WPA3: the AKM that ran (2 WPA2-PSK,
//! 6 PSK-SHA256, 8 WPA3-SAE) and the access point's status or reason code.
//! The link reply body is `[associated u8][bssid 6][ssid_len u8][ssid]`, then
//! `[akm u8]` from such a driver, then the link's history (`tail`). Every
//! field is read only when the reply reaches it; a shorter reply reads as zero
//! there, and a reply without the history reads as lost.

mod connect;
mod join_body;
mod link;
mod tail;

pub use connect::{parse_connect_reply, ConnectResult};
pub use join_body::{encode_join, JoinFlags, AKM_SAE, JOIN_BODY_MAX, JOIN_PASS_MAX};
pub use link::{parse_link, Link};
pub use tail::{LastJoin, LinkEnded};

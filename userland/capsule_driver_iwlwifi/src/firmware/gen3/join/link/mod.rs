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
//! The open port: Ethernet frames to and from the access point through the
//! shared station engine (`nonos_wifi_core::station::LinkStation`), and the
//! `LinkPort` net_core drives it with, the same link protocol it speaks to
//! the RTL8821CE.
//!
//! The station runs `Ccmp::SoftwareTxHwRx`: it protects every transmit
//! frame itself (CCMP header, packet number, MIC) and the firmware sends it
//! as it is; a receive frame the firmware decrypted from its key table is
//! taken as decrypted, and any other protected frame the station decrypts
//! under the pairwise or group key it holds. Every received data frame
//! passes the station's checks (from the BSS, to this station or a group,
//! not fragmented, protected unless EAPOL, not a replay under its key and
//! TID) before the stack sees it. A frame the firmware decrypted reaches
//! here without its MIC (the receive accelerator strips it), so eight bytes
//! stand in for it: the station's hardware path takes the plaintext between
//! the CCMP header and the MIC and never reads the MIC itself.
//!
//! The access point's group key handshakes and repeated message 3s are
//! answered here, protected; a new group key is kept for the serving loop
//! to install. Management frames go to the station too, which judges a
//! deauthentication (taken in the clear only without management frame
//! protection), asks and answers SA Queries, and, ticked at each service
//! with the firmware's missed-beacons notice, ends a link whose access point
//! went away.

mod netif;
mod open;
mod port;
mod receive;
mod state;

pub use port::Port;
pub use state::Link;

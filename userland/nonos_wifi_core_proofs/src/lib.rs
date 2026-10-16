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

//! Proofs for the shared WiFi core: the net_core link protocol (netif) and the
//! station data path, checked against the crate's public API.

// Every module here is a proof or the simulation a proof drives, so the
// crate is empty outside `cargo test`.
#![cfg(test)]

mod ap_sim;
mod bip_gmac_frames;
mod bip_gmac_reading_tests;
mod bip_gmac_refusal_tests;
mod bip_gmac_tests;
mod bip_tests;
mod cipher_select_tests;
mod cipher_sim;
mod country_tests;
mod crypto_kdf_tests;
mod crypto_tests;
mod gcm_key_tests;
mod gcm_tests;
mod gcmp_frame_tests;
mod gcmp_handshake_tests;
mod gcmp_malformed_tests;
mod handshake;
mod hex;
mod mlme;
mod netif;
mod pmksa;
mod pmf_frames;
mod pmf_link_tests;
mod pmf_plain_tests;
mod pmf_query_tests;
mod pmf_sim;
mod prf_tests;
mod protect_mgmt_tests;
mod protect_tests;
mod ptk_len_tests;
mod receive;
mod regdom_mlme_tests;
mod regdom_tests;
mod rsn_key_data_tests;
mod rsn_select_tests;
mod rsn_tests;
mod sa_query_tests;
mod sae;
mod scan_list_bounds_tests;
mod scan_list_tests;
mod soft;
mod softmac;
mod station_counter_tests;
mod station_gcmp_tests;
mod station_tests;
mod supplicant_sim;
mod supplicant_tests;

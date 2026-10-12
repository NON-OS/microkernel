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

#[cfg(test)]
mod ap_sim;
#[cfg(test)]
mod bip_gmac_frames;
#[cfg(test)]
mod bip_gmac_reading_tests;
#[cfg(test)]
mod bip_gmac_refusal_tests;
#[cfg(test)]
mod bip_gmac_tests;
#[cfg(test)]
mod bip_tests;
#[cfg(test)]
mod cipher_select_tests;
#[cfg(test)]
mod cipher_sim;
#[cfg(test)]
mod country_tests;
#[cfg(test)]
mod crypto_tests;
#[cfg(test)]
mod gcm_key_tests;
#[cfg(test)]
mod gcm_tests;
#[cfg(test)]
mod gcmp_handshake_tests;
#[cfg(test)]
mod gcmp_frame_tests;
#[cfg(test)]
mod gcmp_malformed_tests;
#[cfg(test)]
mod handshake_tests;
#[cfg(test)]
mod mlme_tests;
#[cfg(test)]
mod netif_tests;
#[cfg(test)]
mod pmf_frames;
#[cfg(test)]
mod pmf_link_tests;
#[cfg(test)]
mod pmf_plain_tests;
#[cfg(test)]
mod pmf_query_tests;
#[cfg(test)]
mod pmf_sim;
#[cfg(test)]
mod prf_tests;
#[cfg(test)]
mod ptk_len_tests;
#[cfg(test)]
mod protect_tests;
#[cfg(test)]
mod receive_tests;
#[cfg(test)]
mod regdom_mlme_tests;
#[cfg(test)]
mod regdom_tests;
#[cfg(test)]
mod rsn_tests;
#[cfg(test)]
mod sa_query_tests;
#[cfg(test)]
mod sae_tests;
#[cfg(test)]
mod scan_list_tests;
#[cfg(test)]
mod soft;
#[cfg(test)]
mod softmac;
#[cfg(test)]
mod station_gcmp_tests;
#[cfg(test)]
mod station_tests;
#[cfg(test)]
mod supplicant_tests;

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

//! Every proof module, so the crate root stays declarations of real source.

mod onion_client_auth_tests;
mod onion_cache_tests;
mod onion_ntor_tests;
mod onion_pow_tests;
mod onion_desc_tests;
mod onion_ring_tests;
mod keccak_tests;
mod cert_rule_tests;
mod authority_cert_more_tests;
mod authority_cert_tests;
mod backpressure_more_tests;
mod backpressure_tests;
mod base64_encode_tests;
mod base64_live_tests;
mod base64_tests;
mod batch_more_tests;
mod batch_tests;
mod begin_tests;
mod body_step_more_tests;
mod body_step_tests;
mod cell_order_more_tests;
mod cell_order_tests;
mod circuit_answer_more_tests;
mod circuit_answer_tests;
mod circuit_mirror;
mod consensus_header_tests;
mod consensus_live_fixture;
mod consensus_live_more_tests;
mod consensus_live_tests;
mod consensus_relay_tests;
mod destroy_tests;
mod digest_chain_tests;
mod draw_bias_tests;
mod draw_tests;
mod join_more_tests;
mod join_tests;
mod link_step_tests;
mod microdesc_tests;
mod ntor_tests;
mod onion_fixture;
mod onion_layer_tests;
mod path_family_last_tests;
mod path_family_more_tests;
mod guard_pick_tests;
mod path_family_tests;
mod refresh_tests;
mod path_pool;
mod path_through_tests;
mod sendme_pin_more_tests;
mod sendme_pin_tests;
mod sendme_tests;
mod sha1_running_tests;
mod sha1_tests;
mod sha256_more_tests;
mod sha256_tests;
#[path = "shim/socks.rs"]
mod socks;
mod socks_exit_tests;
mod socks_fake;
mod socks_frames;
mod socks_handshake_tests;
mod socks_limit_tests;
mod socks_lost_tests;
mod socks_missed_tests;
mod socks_replay_tests;
mod socks_stream_tests;
mod stream_rule_tests;
mod stream_owner_tests;
mod stream_scope_tests;
mod stream_share_tests;

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

//! Proofs of the install role: package formats, their signatures, the
//! market and escrow checks, and what a package may place.

mod alpine_index_tests;
mod apps_tests;
mod auth_field_tests;
mod auth_refusals;
mod auth_refusals_more;
mod auth_tests;
mod chat_pick_tests;
pub(super) mod deb_chain_tests;
mod deb_file_tests;
mod deb_path_tests;
mod escrow_check_tests;
mod escrow_pin_tests;
mod hash_windows_tests;
mod host_body_tests;
mod http_reply_tests;
mod image_file_tests;
mod index_tests;
mod inflate_bound_tests;
mod install_refusals_tests;
mod kali_anchor_tests;
mod listing_family_tests;
mod manifest_tests;
mod market_tests;
mod member_refusal_tests;
mod mirror_tests;
mod mixnet_name_tests;
mod model_name_tests;
mod pacman_desc_tests;
mod pacman_rsa_tests;
mod pinned_published;
mod pinned_tests;
mod proof_names_tests;
mod route_read_tests;
mod service;
mod store_why_tests;
mod tar_build;
mod tar_link_tests;
mod tar_pax_tests;
mod tar_tests;
mod tar_walk_tests;
mod tier_word_tests;
mod tools_tests;

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

//! The custody tests: kept records, the keyring's owners, words, typed data and
//! retired ops, the vault, the frame.

#[path = "custody_gate_tests.rs"]
mod custody_gate_tests;
#[path = "keep_disk.rs"]
mod keep_disk;
#[path = "keep_refusal_tests.rs"]
mod keep_refusal_tests;
#[path = "keep_status_tests.rs"]
mod keep_status_tests;
#[path = "keep_tests.rs"]
mod keep_tests;
#[path = "kept_names_tests.rs"]
mod kept_names_tests;
#[path = "keyring_bond_tests.rs"]
mod keyring_bond_tests;
#[path = "keyring_ended_tests.rs"]
mod keyring_ended_tests;
#[path = "keyring_owner_rig.rs"]
mod keyring_owner_rig;
#[path = "keyring_owner_tests.rs"]
mod keyring_owner_tests;
#[path = "layout_frame.rs"]
mod layout_frame;
#[path = "layout_tests.rs"]
mod layout_tests;
#[path = "layout_width_tests.rs"]
mod layout_width_tests;
#[path = "replace_tests.rs"]
mod replace_tests;
#[path = "retired_tests.rs"]
mod retired_tests;
#[path = "store_default_tests.rs"]
mod store_default_tests;
#[path = "typed_admit_tests.rs"]
mod typed_admit_tests;
#[path = "typed_check_tests.rs"]
mod typed_check_tests;
#[path = "typed_hash_tests.rs"]
mod typed_hash_tests;
#[path = "typed_live_tests.rs"]
mod typed_live_tests;
#[path = "typed_request_tests.rs"]
mod typed_request_tests;
#[path = "typed_shape_tests.rs"]
mod typed_shape_tests;
#[path = "typed_spec_tests.rs"]
mod typed_spec_tests;
#[path = "typed_word_tests.rs"]
mod typed_word_tests;
#[path = "vault_gate_tests.rs"]
mod vault_gate_tests;
#[path = "vault_read/mod.rs"]
mod vault_read;
#[path = "words_own_tests.rs"]
mod words_own_tests;

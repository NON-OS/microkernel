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

//! The proof a yes in the store asks for (I-B9a), its pure parts mounted
//! beside each other as the store keeps them: the sequence, the words, the
//! GETs a container is put together from, the box check and the offers.

#[path = "../../../capsule_app_store/src/store/proof_box.rs"]
pub mod proof_box;
#[path = "../../../capsule_app_store/src/store/proof_event.rs"]
pub mod proof_event;
#[path = "../../../capsule_app_store/src/store/proof_fail.rs"]
pub mod proof_fail;
#[path = "../../../capsule_app_store/src/store/proof_get.rs"]
pub mod proof_get;
#[path = "../../../capsule_app_store/src/store/proof_offer.rs"]
pub mod proof_offer;
#[path = "../../../capsule_app_store/src/store/proof_said.rs"]
pub mod proof_said;
#[path = "../../../capsule_app_store/src/store/proof_seq.rs"]
pub mod proof_seq;
#[path = "../../../capsule_app_store/src/store/proof_step.rs"]
pub mod proof_step;
#[path = "../../../capsule_app_store/src/store/proof_words.rs"]
pub mod proof_words;

mod prove_parts;
mod prove_refused;
mod prove_said;
mod prove_seq;
mod prove_status;

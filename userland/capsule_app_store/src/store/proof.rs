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

//! A proof of how an installed Linux app ran (I-B9a): the store asks for the
//! headless prover on the person's yes, sends PROVE, follows it, and holds
//! the container it makes, here, unsent. Each part is a file beside this
//! one; the pure ones are market_proofs' too, mounted the same way.

#[path = "proof_box.rs"]
pub mod proof_box;
#[path = "proof_call.rs"]
pub mod proof_call;
#[path = "proof_debug.rs"]
pub mod proof_debug;
#[path = "proof_event.rs"]
pub mod proof_event;
#[path = "proof_fail.rs"]
pub mod proof_fail;
#[path = "proof_fetch.rs"]
pub mod proof_fetch;
#[path = "proof_get.rs"]
pub mod proof_get;
#[path = "proof_held.rs"]
pub mod proof_held;
#[path = "proof_installed.rs"]
pub mod proof_installed;
#[path = "proof_keys.rs"]
pub mod proof_keys;
#[path = "proof_offer.rs"]
pub mod proof_offer;
#[path = "proof_perform.rs"]
pub mod proof_perform;
#[path = "proof_run.rs"]
pub mod proof_run;
#[path = "proof_said.rs"]
pub mod proof_said;
#[path = "proof_seq.rs"]
pub mod proof_seq;
#[path = "proof_state.rs"]
pub mod proof_state;
#[path = "proof_step.rs"]
pub mod proof_step;
#[path = "proof_watch.rs"]
pub mod proof_watch;
#[path = "proof_words.rs"]
pub mod proof_words;

pub use proof_state::Proofs;

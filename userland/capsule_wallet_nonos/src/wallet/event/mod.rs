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

pub mod address_text;
pub(crate) mod custody;
mod escape;
mod etna_click;
mod etna_click_receive;
mod etna_click_views;
mod etna_scroll;
pub(crate) mod hex_digit;
pub(crate) mod locking;
mod on_event;
mod on_key;
mod on_pointer;
pub(crate) mod probe;
mod recipient;
pub(crate) mod stake;
pub(crate) mod swap;

pub use custody::custody_guard::may_replace;
pub use custody::export_key::toggle_export;
pub use custody::import::{import_input, toggle_import};
pub use custody::recover::toggle_recover;
pub use custody::recover_input::recover_input;
pub use custody::replace_rule::gave_up as vault_gave_up;
pub use hex_digit::hex_digit;
pub use locking::lock::lock;
pub use on_event::on_event;
pub use probe::probe_tick::probe_tick;
pub use probe::tx_freshen::take_nonce_and_fee;
pub use recipient::recipient;
pub use stake::stake_guard::refusal as stake_refusal;
pub use stake::stake_input::stake_input;
pub use stake::stake_set::clear as stake_clear;
pub use stake::stake_set::set_max as stake_set_max;
pub use stake::stake_wei::stake_wei;

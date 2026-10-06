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
mod backup;
mod custody_guard;
mod escape;
mod etna_click;
mod etna_scroll;
mod export_key;
mod generate;
mod hex_digit;
pub mod idle_lock;
mod import;
mod keep;
mod keep_plan;
mod keyring_says;
mod lock;
mod on_event;
mod on_key;
mod on_pointer;
mod probe_tick;
mod reading;
mod recipient;
mod recover;
mod replace_rule;
mod stake_amount;
mod stake_guard;
mod stake_input;
mod stake_set;
mod stake_wei;
mod swap_amount;
mod swap_input;
mod swap_pair;
pub(crate) mod swap_quote;
mod tx_freshen;

pub use custody_guard::may_replace;
pub use export_key::toggle_export;
pub use hex_digit::hex_digit;
pub use import::{import_input, toggle_import};
pub use lock::lock;
pub use on_event::on_event;
pub use probe_tick::probe_tick;
pub use recipient::recipient;
pub use recover::{recover_input, toggle_recover};
pub use replace_rule::gave_up as vault_gave_up;
pub use stake_guard::refusal as stake_refusal;
pub use stake_input::stake_input;
pub use stake_set::clear as stake_clear;
pub use stake_set::set_max as stake_set_max;
pub use stake_wei::stake_wei;
pub use tx_freshen::take_nonce_and_fee;

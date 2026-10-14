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
//! Signing a reviewed payment and following it onto the chain.

use crate::wallet::state::State;

#[path = "confirm_follow.rs"]
mod confirm_follow;
#[path = "confirm_hold.rs"]
mod confirm_hold;
#[path = "confirm_sign.rs"]
mod confirm_sign;
pub use confirm_follow::{awaiting, follow};
pub use confirm_hold::held_back;
pub use confirm_sign::sign_draft;

/// Sign the reviewed payment and record it for the broadcast.
pub fn sign(state: &mut State) -> Result<(), &'static str> {
    let draft = state.send.draft.clone().ok_or("Nothing to confirm. Review the payment again.")?;
    sign_draft(state, &draft, b"send")
}

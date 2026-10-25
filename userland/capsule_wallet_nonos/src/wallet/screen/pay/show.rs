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
//! A public payment from this account: ETH, NOX or USDC, on the network

//! The payment screen up now: the form, its review, or what was sent.

use nonos_app_skeleton::PaintBuffer;

use super::{done, form, review};
use crate::wallet::send::{STAGE_FORM, STAGE_REVIEW};
use crate::wallet::state::State;

pub fn show(state: &State, fb: &mut PaintBuffer) {
    match state.send.stage {
        STAGE_FORM => form::form(state, fb),
        STAGE_REVIEW => review::review(state, fb),
        _ => done::done(state, fb),
    }
}

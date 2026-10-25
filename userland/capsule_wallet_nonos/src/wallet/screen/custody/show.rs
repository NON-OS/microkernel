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
//! 02  The key itself: importing a private key, restoring from recovery
//! words, and showing this account's private key. Each owns the keyboard
//! while it is up, and what is typed or shown is wiped when it closes.

use nonos_app_skeleton::PaintBuffer;

use super::{export::export, import::import, recover::recover};
use crate::wallet::state::{State, VIEW_EXPORT, VIEW_IMPORT};

pub fn show(state: &State, fb: &mut PaintBuffer) {
    match state.view {
        VIEW_IMPORT => import(state, fb),
        VIEW_EXPORT => export(state, fb),
        _ => recover(state, fb),
    }
}

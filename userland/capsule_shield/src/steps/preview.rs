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

//! The private address an open under way will answer, shown before the store is made.

use std::sync::Mutex;

/// The private address of the open under way, derived from its words or key before the store
/// is made, so the wallet shows it while the rest runs. Public by nature: it is the address
/// others pay to.
static PREVIEW: Mutex<Option<String>> = Mutex::new(None);

/// Keep the private address the open under way will answer.
pub fn preview(address: Option<String>) {
    if let Ok(mut p) = PREVIEW.lock() {
        *p = address;
    }
}

/// The private address of the open under way, once derived.
pub fn previewed() -> Option<String> {
    PREVIEW.lock().ok().and_then(|p| p.clone())
}

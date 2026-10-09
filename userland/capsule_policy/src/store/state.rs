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

use core::sync::atomic::{AtomicU32, Ordering};

use spin::Mutex;

use super::defaults::store;
use super::types::Store;

pub static STORE: Mutex<Store> = Mutex::new(store());

/// Moves on every value set, so the keeper can tell a store that changed
/// since it last wrote the record from one that did not.
pub static CHANGES: AtomicU32 = AtomicU32::new(0);

pub fn changed() {
    CHANGES.fetch_add(1, Ordering::Relaxed);
}

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

//! The logger's start and its shutdown wipe.

use super::state::{LogManager, LOGGER};

pub fn init() {
    let mut l = LOGGER.lock();
    let mgr = LogManager::new();
    // Only x86 has a text-mode console to mirror the log onto.
    #[cfg(target_arch = "x86_64")]
    let mgr = {
        let mut mgr = mgr;
        mgr.add_backend(alloc::boxed::Box::new(crate::log::backend::VgaBackend));
        mgr
    };
    *l = Some(mgr);
}

/// The shutdown wipe's: overwrite what the logger holds in RAM. It never
/// waits for the lock, since a holder may be the very CPU that is shutting
/// down; a log held at that moment keeps its text until power goes.
pub fn wipe_ram_log() -> bool {
    match LOGGER.try_lock() {
        Some(mut l) => {
            if let Some(mgr) = l.as_mut() {
                mgr.wipe();
            }
            true
        }
        None => false,
    }
}

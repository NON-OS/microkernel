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

//! The entry word itself, which `SPECTRE_FROM_EL0` in vectors.S loads by
//! symbol on every exception from EL0.

use core::sync::atomic::{AtomicU32, Ordering};

use super::entry_word::merge;

/// Zero until a CPU publishes, which every CPU does in `security::init_all`
/// before it can take an exception from EL0.
#[no_mangle]
static NONOS_SPECTRE_EL0_ENTRY: AtomicU32 = AtomicU32::new(0);

/// Fold this CPU's word into the shared one. Words only ever gain bits and
/// counts, so a CPU that started earlier never loses a mitigation.
pub(super) fn publish(word: u32) {
    let _ = NONOS_SPECTRE_EL0_ENTRY
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |now| Some(merge(now, word)));
}

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

//! Whether a device table write moves an entry into passthrough or out of
//! it: the one fact the posture line's identity count needs from each
//! write. Pure, so the proofs hold it against every entry kind.

use super::dte::{mode_of, passthrough, Dte};

const VALID: u64 = 1 << 0;
const TRANSLATION_VALID: u64 = 1 << 1;
const READ: u64 = 1 << 61;
const WRITE: u64 = 1 << 62;

/// The entry lets the device reach physical memory untranslated: valid, with
/// read or write allowed, and either no translation information or Mode 0,
/// which walks no table. Judged by what the unit does with the entry rather
/// than by matching `passthrough()`, so no encoding of it goes uncounted.
pub const fn is_passthrough(entry: Dte) -> bool {
    let word = entry[0];
    let untranslated = word & TRANSLATION_VALID == 0 || mode_of(entry) == 0;
    word & VALID != 0 && word & (READ | WRITE) != 0 && untranslated
}

// The entry format's own passthrough is counted, checked as the kernel builds.
const _: () = assert!(is_passthrough(passthrough()));

/// +1 when a write makes an entry passthrough, -1 when it ends one, 0 when
/// it does neither (passthrough rewritten as passthrough included, as when
/// bring-up names an id twice).
pub fn identity_change(before: Dte, after: Dte) -> i8 {
    is_passthrough(after) as i8 - is_passthrough(before) as i8
}

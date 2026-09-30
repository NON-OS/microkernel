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

/*
 * The tier `qwen` runs when none is named: the one chosen at setup or in
 * Settings when it is a tier this terminal knows, else the first.
 */

use nonos_policy_proto::Field;

use super::tiers::pick;
use crate::term::identity::Cached;

static CHOSEN: Cached = Cached::new(Field::QwenTier, tier_len);

/* A tier word is lowercase letters, digits, `.` and `-`; anything else ends it. */
fn tier_len(b: &[u8]) -> usize {
    let word = |c: &u8| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'.' || *c == b'-';
    b.iter().position(|c| !word(c)).unwrap_or(b.len())
}

pub fn chosen() -> &'static [u8] {
    pick(CHOSEN.get())
}

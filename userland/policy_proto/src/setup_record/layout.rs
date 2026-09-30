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
 * Where the record lives and how its versions are laid out.
 *
 * Version 1 holds keyboard, time zone and wallpaper. Version 2 adds the name
 * and the Qwen tier, each as a length byte and a field padded with zeros.
 */

use super::rules::{NAME_MAX, TIER_MAX};

pub const SETUP_DIR: &[u8] = b"/nonos/setup";
pub const ANSWERS_PATH: &[u8] = b"/nonos/setup/answers";
pub const DONE_PATH: &[u8] = b"/nonos/setup/done";

pub const DONE: [u8; 4] = *b"NSD1";

pub(super) const MAGIC_V1: [u8; 4] = *b"NSA1";
pub(super) const MAGIC_V2: [u8; 4] = *b"NSA2";

/* Magic, keyboard, time zone, wallpaper. */
pub const ANSWERS_V1_LEN: usize = 7;
pub(super) const NAME_AT: usize = ANSWERS_V1_LEN;
pub(super) const TIER_AT: usize = NAME_AT + 1 + NAME_MAX;
/* The version setup writes now, and the most a reader need ask vfs for. */
pub const ANSWERS_LEN: usize = TIER_AT + 1 + TIER_MAX;

/* Whether `raw` is the marker setup writes once it has kept its answers. */
pub fn is_done(raw: &[u8]) -> bool {
    raw == DONE
}

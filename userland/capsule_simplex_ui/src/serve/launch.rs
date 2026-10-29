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

//! The core started from its listing, with the one word the start gives it.

use nonos_libc::mk_app_launch_with;

use crate::model::start::Start;

/// The core's own listing, which this window is paired with.
const CORE: &[u8] = b"linux.kali.nonos-simplex-core";

/// Asks MkAppLaunch for the core, with `word` after its args when given, and
/// tells `start` the answer at `now`.
pub(super) fn launch(start: &mut Start, word: Option<&str>, now: i64) {
    let rc = mk_app_launch_with(CORE, word.unwrap_or("").as_bytes());
    start.launched(rc, now);
}

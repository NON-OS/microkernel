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

//! Starting what a queued run asked for, said on the log.

use crate::sys::serial::{print, println};
use crate::userspace::capsule_linux::spawn_run;

/// Start `package`'s program. A quiet run is said without its package: a
/// shipped tier a terminal asked for is named nowhere on the log.
pub(super) fn run(package: &str, quiet: bool) {
    let said: &[u8] = match spawn_run(package) {
        Ok(_) => b"[LINUX-RUN] started ",
        Err(_) => b"[LINUX-RUN] refused ",
    };
    print(said);
    println(if quiet { b"a Qwen window" } else { package.as_bytes() });
}

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

//! A loaded capsule's arguments, NUL-separated, onto its process.

pub(super) fn set_argv(pid: u32, args: &[u8]) {
    if !args.is_empty() {
        let argv: alloc::vec::Vec<alloc::string::String> = args
            .split(|&b| b == 0)
            .filter(|s| !s.is_empty())
            .map(|s| alloc::string::String::from_utf8_lossy(s).into_owned())
            .collect();
        crate::process::with_process(pid, |pcb| *pcb.argv.lock() = argv);
    }
}

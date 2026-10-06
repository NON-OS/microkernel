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

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::{store_status, store_was_read};

use crate::term::state::State;
use crate::term::util::format_u64;

// Report whether the on-device store behind the installed packages is
// usable, so a failed install can be told apart from a broken store. A store
// that loaded with a damaged entry left out (9) is usable; only the codes that
// mean no disk was read say the boot has none (app_skeleton vfs store_disk).
pub(super) fn status(state: &mut State) -> bool {
    match store_status() {
        Ok(0) => {
            state.scrollback.push_line(b"store healthy");
            true
        }
        Ok(9) => {
            state.scrollback.push_line(
                b"store loaded; a damaged entry was left out",
            );
            true
        }
        Ok(code) if !store_was_read(code) => {
            state.scrollback.push_error(b"no NONOS disk on this boot: packages are not kept");
            false
        }
        Ok(code) => {
            let mut num = [0u8; 24];
            let k = format_u64(code as u64, &mut num);
            let mut line = Vec::with_capacity(12 + k);
            line.extend_from_slice(b"store error ");
            line.extend_from_slice(&num[..k]);
            state.scrollback.push_error(&line);
            false
        }
        Err(_) => {
            state.scrollback.push_error(b"store error: vfs unreachable");
            false
        }
    }
}

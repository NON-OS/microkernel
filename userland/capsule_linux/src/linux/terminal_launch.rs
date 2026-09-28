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

//! Reading the program the terminal's `linux` command names.

use alloc::vec::Vec;

use super::launch::Launch;
use super::origin::Origin;
use crate::linux::file::{key, store_read, visible};
use crate::linux::guest::Links;
use crate::linux::start::say;

pub(super) fn launch(program: &[u8], mut args: Vec<Vec<u8>>, max_image: u32) -> Option<Launch> {
    if !super::settle::wait_settled() {
        say(b"[LINUX] the store never settled\n");
        return None;
    }
    let named = match program.first() {
        Some(b'/') => visible(b"/", program),
        _ => visible(b"/bin", program),
    };
    let path = Links::load().follow(named.clone(), true);
    if path != named {
        let typed = named.rsplit(|b| *b == b'/').next().unwrap_or(&named);
        args.insert(0, typed.to_vec());
    }
    match store_read(&key(&path), max_image) {
        Ok(bytes) => Some(Launch { path, bytes, origin: Origin::Store, args }),
        Err(_) => {
            let line = alloc::format!(
                "linux: no program {} in the Linux tree\n",
                alloc::string::String::from_utf8_lossy(&named)
            );
            say(line.as_bytes());
            None
        }
    }
}

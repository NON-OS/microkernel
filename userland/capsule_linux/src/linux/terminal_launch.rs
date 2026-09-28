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

//! Reading the program the terminal's `linux` command names, and saying
//! what stopped it when it cannot be read: what the store answered, never a
//! guess at why.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::launch::Launch;
use super::origin::Origin;
use crate::linux::file::{key, store_read, store_stat, visible};
use crate::linux::guest::Links;
use crate::linux::say::say;

pub(super) fn launch(program: &[u8], mut args: Vec<Vec<u8>>, max: u32) -> Option<Launch> {
    if !super::settle::wait_settled() {
        say(b"[LINUX] the store never settled\n");
        return None;
    }
    let named = match program.first() {
        Some(b'/') => visible(b"/", program),
        _ => visible(b"/bin", program),
    };
    let links = Links::try_load();
    let path = links.as_ref().map_or_else(|_| named.clone(), |l| l.follow(named.clone(), true));
    if path != named {
        let typed = named.rsplit(|b| *b == b'/').next().unwrap_or(&named);
        args.insert(0, typed.to_vec());
    }
    let why = match store_read(&key(&path), max) {
        Ok(bytes) => return Some(Launch { path, bytes, origin: Origin::Store, args }),
        Err(why) => why,
    };
    let line = match store_stat(&key(&path)) {
        Ok((size, _)) => {
            format!("linux: {} is there ({size} bytes) but unread: {why}\n", text(&path))
        }
        Err(_) => match nonos_app_skeleton::clients::vfs::store_status() {
            Ok(0) => format!("linux: cannot find {} in the Linux tree\n", text(&named)),
            Ok(code) => {
                format!("linux: cannot find {}: the store reports error {code}\n", text(&named))
            }
            Err(e) => {
                format!("linux: cannot find {}: the store does not answer ({e})\n", text(&named))
            }
        },
    };
    say(line.as_bytes());
    if let (Err(why), Ok(_)) = (links, store_stat(&key(Links::TABLE))) {
        say(format!("linux: link table {} unread: {why}\n", text(Links::TABLE)).as_bytes());
    }
    None
}

fn text(path: &[u8]) -> String {
    String::from_utf8_lossy(path).into_owned()
}

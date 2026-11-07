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

//! Keeping the image cache within its budget (prune.rs): each install of an
//! image stamps when it was used and which listing it is, then drops the
//! least recently used images whose apps are not installed until the cache
//! fits. An image whose stamp or listing cannot be read counts as unused
//! and not installed, so it goes first; one whose size cannot be read is
//! left alone.

use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;
use nonos_libc::{mk_debug, mk_getpid};

use super::super::record::closure::closure;
use super::super::timing;
use super::named::hex;
use super::prune::{budget, evict, read_stamp, stamp, Cached};
use crate::linux::file::{key, store_read};

const DIR: &[u8] = b"/nonos/linux/image-cache";
const SETTING: &[u8] = b"/etc/nonos-image-cache-max";

fn at(pin_hex: &str, leaf: &str) -> Vec<u8> {
    [DIR, b"/", pin_hex.as_bytes(), b"/", leaf.as_bytes()].concat()
}

/// `pin` was used now, by the listing `pkg`.
pub fn touch(pin: &[u8; 32], pkg: &str) {
    let (pid, h) = (mk_getpid(), hex(pin));
    let _ = vfs::write_file(pid, &at(&h, "used"), &stamp(timing::now()));
    let _ = vfs::write_file(pid, &at(&h, "listing"), pkg.as_bytes());
}

/// Drop what the budget says, never `keep`.
pub fn prune(keep: &[u8; 32]) {
    let pid = mk_getpid();
    let Ok(paths) = vfs::list_paths(pid, DIR) else { return };
    let pins: BTreeSet<String> = paths.iter().filter_map(|p| pin_of(p)).collect();
    let cache: Vec<Cached> = pins.iter().filter_map(|h| cached(pid, h)).collect();
    let setting = store_read(&key(SETTING), 32).ok();
    for pin in evict(&cache, budget(setting.as_deref()), keep) {
        let dir = [DIR, b"/", hex(&pin).as_bytes()].concat();
        if vfs::rmdir(pid, &dir, true).is_ok() {
            let line = alloc::format!("[LINUX] image: dropped a cached image, {}\n", hex(&pin));
            let _ = mk_debug(line.as_ptr(), line.len());
        }
    }
}

/// The 64-hex pin a cache path names, after the cache's own directory.
fn pin_of(path: &str) -> Option<String> {
    let rest = path.strip_prefix(core::str::from_utf8(DIR).ok()?)?.trim_start_matches('/');
    let h = rest.split('/').next()?;
    (h.len() == 64 && h.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))).then(|| String::from(h))
}

fn cached(pid: u32, h: &str) -> Option<Cached> {
    let mut pin = [0u8; 32];
    for (i, b) in pin.iter_mut().enumerate() {
        *b = u8::from_str_radix(h.get(2 * i..2 * i + 2)?, 16).ok()?;
    }
    let dir = [DIR, b"/", h.as_bytes()].concat();
    let (_, _, bytes, _) = vfs::dirstat(pid, &dir).ok()?;
    let used = vfs::read_file(pid, &at(h, "used"), 8).map_or(0, |r| read_stamp(&r));
    let listing = vfs::read_file(pid, &at(h, "listing"), 128).ok();
    let installed = listing
        .and_then(|l| String::from_utf8(l).ok())
        .is_some_and(|name| closure(&name).is_some());
    Some(Cached { pin, bytes, used, installed })
}

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

//! The one line an installer leaves on why its job failed: which package,
//! which check. Only in memory, for its listing, shown once that job has
//! failed and gone at restart or when the next starts. Never printed,
//! logged or stored: what the person installs is their own.

use alloc::collections::BTreeMap;
use alloc::string::String;

use spin::Mutex;

use super::status::{get, Stage};

/// The longest line kept, in bytes; as many listings as the stage table.
pub(crate) const MAX: usize = 120;
const CAP: usize = 32;

/// `pid`, the installer or remover, alone may leave the listing's line.
struct Held {
    pid: u32,
    line: Option<([u8; MAX], usize)>,
}

static HELD: Mutex<BTreeMap<String, Held>> = Mutex::new(BTreeMap::new());

/// The listing's next job starts, run by `pid` (0 before one is spawned):
/// its old line goes. When full, one no longer moving gives way.
pub(super) fn bind(listing: &str, pid: u32) {
    let mut held = HELD.lock();
    if held.len() >= CAP && !held.contains_key(listing) {
        let moving = |k: &String| matches!(get(k), Some(Stage::Running(_) | Stage::Removing(_)));
        if let Some(k) = held.keys().find(|k| !moving(k)).cloned() {
            held.remove(&k);
        }
    }
    held.insert(String::from(listing), Held { pid, line: None });
}

/// `pid` leaves `line`, printable ASCII of at most MAX bytes. False unless
/// `pid` is running the job of the listing it was bound to.
pub(crate) fn tell(pid: u32, line: &[u8]) -> bool {
    let listing = HELD.lock().iter().find(|(_, h)| h.pid == pid).map(|(k, _)| k.clone());
    let Some(listing) = listing.filter(|l| get(l).and_then(Stage::pid) == Some(pid)) else {
        return false;
    };
    let (mut kept, n) = ([0u8; MAX], line.len().min(MAX));
    kept[..n].copy_from_slice(&line[..n]);
    let mut held = HELD.lock();
    held.get_mut(&listing).filter(|h| h.pid == pid).map(|h| h.line = Some((kept, n))).is_some()
}

/// The line for `listing` into `out`, and its length: 0 unless its job failed.
pub(crate) fn read(listing: &str, out: &mut [u8; MAX]) -> usize {
    if !matches!(get(listing), Some(Stage::Failed(_))) {
        return 0;
    }
    let Some((line, n)) = HELD.lock().get(listing).and_then(|h| h.line) else { return 0 };
    out[..n].copy_from_slice(&line[..n]);
    n
}

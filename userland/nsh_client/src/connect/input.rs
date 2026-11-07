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

//! What `nsh` takes in: the system's randomness, and the keys typed.

use alloc::vec::Vec;
use std::io::Read;
use std::sync::mpsc::{channel, Receiver};

/// The system's randomness.
pub fn urandom(out: &mut [u8]) -> bool {
    std::fs::File::open("/dev/urandom").and_then(|mut f| f.read_exact(out)).is_ok()
}

/// Read `input` on its own thread, a chunk at a time; an empty chunk is
/// its end.
pub fn keys(mut input: impl Read + Send + 'static) -> Receiver<Vec<u8>> {
    let (tx, rx) = channel();
    std::thread::spawn(move || {
        let mut buf = [0u8; 4096];
        while let Ok(n) = input.read(&mut buf) {
            if tx.send(buf[..n].to_vec()).is_err() || n == 0 {
                break;
            }
        }
    });
    rx
}

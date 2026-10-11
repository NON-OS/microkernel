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

//! The entropy source the station address is drawn from. The driver's promise
//! is that with no entropy the card is never brought on the air under the
//! address in its NVM; the tests switch the source off to hold it to that.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, MutexGuard};

static AVAILABLE: AtomicBool = AtomicBool::new(true);
static TURN: Mutex<()> = Mutex::new(());

/// Holds the switch in one position until dropped. The switch is process
/// wide and tests run in parallel, so each test that draws an address takes
/// its turn through this; dropping it puts entropy back on.
pub struct Entropy {
    _turn: MutexGuard<'static, ()>,
}

impl Drop for Entropy {
    fn drop(&mut self) {
        AVAILABLE.store(true, Ordering::SeqCst);
    }
}

pub fn entropy(available: bool) -> Entropy {
    let turn = TURN.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    AVAILABLE.store(available, Ordering::SeqCst);
    Entropy { _turn: turn }
}

/// A fixed pattern when entropy is on, so a test can recognise the drawn
/// address; a refusal when it is off.
pub fn crypto_random(ptr: *mut u8, len: usize) -> i64 {
    if !AVAILABLE.load(Ordering::SeqCst) {
        return -1;
    }
    for i in 0..len {
        // SAFETY: the caller hands `len` writable bytes, as the real call requires.
        unsafe { *ptr.add(i) = 0xC3 ^ (i as u8) };
    }
    len as i64
}

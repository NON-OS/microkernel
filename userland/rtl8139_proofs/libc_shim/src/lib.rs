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

//! What the included driver files take from `nonos_libc`.
//!
//! The part is reached only through `mk_pio_read` and `mk_pio_write`, so a
//! model behind those two calls answers in the test's own thread. The
//! release calls record what they were handed: a failed bring-up attempt
//! must give back every grant and the claim, or the next attempt finds the
//! device still held.

use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, MutexGuard};

/// A part on the other side of the port grant. `read` answers `None` and
/// `write` answers `false` to refuse the call the way the broker would.
pub trait Port {
    fn read(&mut self, offset: u16, width: u8) -> Option<u32>;
    fn write(&mut self, offset: u16, width: u8, value: u32) -> bool;
}

thread_local! {
    static PORT: RefCell<Option<Box<dyn Port>>> = const { RefCell::new(None) };
    static GIVEN_BACK: RefCell<Vec<(&'static str, u64)>> = const { RefCell::new(Vec::new()) };
}

/// Detaches the model when dropped, so a later test on the same thread
/// starts with nothing behind the port.
pub struct Attached;

impl Drop for Attached {
    fn drop(&mut self) {
        PORT.with(|p| *p.borrow_mut() = None);
    }
}

pub fn attach(port: Box<dyn Port>) -> Attached {
    PORT.with(|p| *p.borrow_mut() = Some(port));
    Attached
}

pub fn mk_pio_read(_grant: u64, offset: u16, width: u8, out: *mut u32) -> i64 {
    let answer = PORT.with(|p| p.borrow_mut().as_mut().and_then(|port| port.read(offset, width)));
    match answer {
        Some(value) => {
            /*
             * SAFETY: the driver passes a reference to a live u32, as the
             * real call requires.
             */
            unsafe { *out = value };
            0
        }
        None => -1,
    }
}

pub fn mk_pio_write(_grant: u64, offset: u16, width: u8, value: u32) -> i64 {
    let taken = PORT.with(|p| {
        p.borrow_mut().as_mut().is_some_and(|port| port.write(offset, width, value))
    });
    if taken {
        0
    } else {
        -1
    }
}

static ENTROPY: AtomicBool = AtomicBool::new(true);
static TURN: Mutex<()> = Mutex::new(());

/// Holds the entropy switch in one position until dropped. The switch is
/// process-wide and the tests run in parallel, so every test that draws an
/// address takes its turn through this; dropping it puts entropy back on.
pub struct Entropy {
    _turn: MutexGuard<'static, ()>,
}

impl Drop for Entropy {
    fn drop(&mut self) {
        ENTROPY.store(true, Ordering::SeqCst);
    }
}

pub fn entropy(available: bool) -> Entropy {
    let turn = TURN.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    ENTROPY.store(available, Ordering::SeqCst);
    Entropy { _turn: turn }
}

/// Fills `len` bytes with a fixed pattern when entropy is on; answers a
/// refusal when it is off.
pub fn crypto_random(ptr: *mut u8, len: usize) -> i64 {
    if !ENTROPY.load(Ordering::SeqCst) {
        return -1;
    }
    for i in 0..len {
        /*
         * SAFETY: the caller hands a buffer of `len` writable bytes, as the
         * real call requires.
         */
        unsafe { *ptr.add(i) = 0x5A ^ (i as u8) };
    }
    len as i64
}

fn give_back(call: &'static str, id: u64) -> i64 {
    GIVEN_BACK.with(|g| g.borrow_mut().push((call, id)));
    0
}

/// Every release the driver made on this thread, as (call, id), in order.
pub fn given_back() -> Vec<(&'static str, u64)> {
    GIVEN_BACK.with(|g| g.borrow().clone())
}

pub fn mk_device_release(device_id: u64) -> i64 {
    give_back("device_release", device_id)
}
pub fn mk_dma_unmap(grant: u64) -> i64 {
    give_back("dma_unmap", grant)
}
pub fn mk_pio_release(grant: u64) -> i64 {
    give_back("pio_release", grant)
}

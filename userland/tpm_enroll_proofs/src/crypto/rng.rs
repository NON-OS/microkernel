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

//! Session nonces: the kernel fills them from its seeded RNG, and refuses
//! when it is not seeded; the host fills them from its own.

use std::io::Read;

pub fn fill_random_bytes(buf: &mut [u8]) {
    let mut f = std::fs::File::open("/dev/urandom").expect("host rng");
    f.read_exact(buf).expect("host rng read");
}

thread_local! {
    static REFUSE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Makes this thread's refusing draws refuse, as the kernel's do before the
/// RNG is seeded.
#[cfg(test)]
pub fn refuse_draws(on: bool) {
    REFUSE.with(|r| r.set(on));
}

/// The kernel's refusing draw: the host's generator, unless a test refused it.
pub(crate) fn fill_random_bytes_secure(buf: &mut [u8]) -> Result<(), ()> {
    if REFUSE.with(|r| r.get()) {
        return Err(());
    }
    fill_random_bytes(buf);
    Ok(())
}
